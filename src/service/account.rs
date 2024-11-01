use amazing::{database, error::Error, AppResult};
use axum::http::StatusCode;
use num_traits::cast::FromPrimitive;
use serde::{Deserialize, Serialize};
use sqlx::types::Decimal;
use validator::Validate;

use super::{
    action_type::{ActionType, Change},
    asset_type::AssetType,
};

#[derive(Deserialize, Validate, Debug)]
pub struct AccountRequest {
    #[validate(range(min = 1, message = "请输入有效的user_id"))]
    pub user_id: i32,
    #[validate(range(min = 1, message = "请输入有效的asset_type_id"))]
    pub asset_type_id: Option<i32>,
    #[validate(range(min = 1, message = "请输入有效的action_type_id"))]
    pub action_type_id: Option<i32>,
    #[validate(range(min = 0.000001, message = "最小值为0.000001"))]
    pub amount: Option<f64>,
    #[validate(length(min = 32, message = "可选参数（至少32个字符）"))]
    pub order_number: Option<String>,
    #[validate(length(min = 1, message = "可选参数（不能为空）"))]
    pub description: Option<String>,
}

impl AccountRequest {
    fn custom_error(message: &str) -> AppResult<()> {
        Err(Error::Custom(
            StatusCode::UNPROCESSABLE_ENTITY,
            message.to_string(),
        ))
    }

    pub async fn validate_asset_type_id(&self) -> AppResult<()> {
        if self.asset_type_id.is_none() || !AssetType::is_active(self.asset_type_id.unwrap()).await
        {
            return Self::custom_error("请输入有效的asset_type_id");
        }
        Ok(())
    }

    pub async fn validate_action_type_id(&self) -> AppResult<()> {
        if self.action_type_id.is_none()
            || !ActionType::is_active(self.action_type_id.unwrap()).await
        {
            return Self::custom_error("请输入有效的action_type_id");
        }
        Ok(())
    }

    // 验证 amount 是否有效
    // 数据库中的金额相关字段为 Decimal(18, 6)
    // 大于6位小数会丢失精度
    pub async fn validate_amount(&self) -> AppResult<()> {
        if self.amount.is_none() || Decimal::from_f64(self.amount.unwrap()).unwrap().scale() > 6 {
            return Self::custom_error("请输入有效的amount（最多允许6位小数）");
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn validate(&self) -> AppResult<()> {
        self.validate_asset_type_id()
            .await
            .and(self.validate_action_type_id().await)
            .and(self.validate_amount().await)
    }
}

#[derive(Serialize)]
pub struct Account {
    pub id: i32,
    pub user_id: i32,
    pub asset_type_id: i32,
    pub available_balance: Decimal,
    pub frozen_balance: Decimal,
    pub total_income: Decimal,
    pub total_expense: Decimal,
    pub is_active: bool,
}

impl Account {
    // 资产账户是否存在
    #[allow(dead_code)]
    pub async fn is_exists(user_id: i32, asset_type_id: i32) -> bool {
        if let Ok(Some(is_exist)) = sqlx::query_scalar!(
            "select exists(select 1 from account where user_id = $1 and asset_type_id = $2)",
            user_id,
            asset_type_id
        )
        .fetch_one(database::conn())
        .await
        {
            return is_exist;
        }
        false
    }

    // 资产账户是否启用
    #[allow(dead_code)]
    pub async fn is_active(user_id: i32, asset_type_id: i32) -> bool {
        if let Ok(Some(is_exist)) = sqlx::query_scalar!(
            "select exists(select 1 from account where user_id = $1 and asset_type_id = $2 and is_active = true)",
            user_id,
            asset_type_id
        )
        .fetch_one(database::conn())
        .await
        {
            return is_exist;
        }
        false
    }

    pub async fn create(account_request: &AccountRequest) -> AppResult<()> {
        account_request.validate_asset_type_id().await?;
        if Account::is_exists(
            account_request.user_id,
            account_request.asset_type_id.unwrap(),
        )
        .await
        {
            return Err(Error::Custom(
                StatusCode::CONFLICT,
                "账户已存在".to_string(),
            ));
        }
        sqlx::query!(
            r#"insert into account (user_id, asset_type_id, is_active) values ($1, $2, true)"#,
            account_request.user_id,
            account_request.asset_type_id.unwrap()
        )
        .execute(database::conn())
        .await?;
        Ok(())
    }

    pub async fn info(account_request: &AccountRequest) -> AppResult<Vec<Account>> {
        let asset_type_ids = match account_request.asset_type_id {
            Some(asset_type_id) => {
                account_request.validate_asset_type_id().await?;
                vec![asset_type_id]
            }
            None => AssetType::get_active_ids().await?,
        };
        if !asset_type_ids.is_empty() {
            let accounts = sqlx::query_as!(
                Account,
                r#"select id, user_id, asset_type_id, available_balance, frozen_balance, total_income, total_expense, is_active from account where user_id = $1 and asset_type_id = any($2)"#,
                account_request.user_id,
                &asset_type_ids
            )
            .fetch_all(database::conn())
            .await?;
            if !accounts.is_empty() {
                return Ok(accounts);
            }
        }
        Err(Error::Custom(
            StatusCode::NOT_FOUND,
            "账户不存在".to_string(),
        ))
    }

    pub async fn update_balance(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        account_request: &AccountRequest,
    ) -> AppResult<()> {
        account_request.validate().await?;
        if !Account::is_active(
            account_request.user_id,
            account_request.asset_type_id.unwrap(),
        )
        .await
        {
            return Err(Error::Custom(
                StatusCode::FORBIDDEN,
                "账户未启用".to_string(),
            ));
        }
        let amount = account_request.amount.unwrap();
        let action_type = ActionType::fetch_one(account_request.action_type_id.unwrap()).await?;
        let account = sqlx::query_as!(
            Account,
            r#"update account
                set available_balance = available_balance + $3,
                frozen_balance = frozen_balance + $4,
                total_income = total_income + $5,
                total_expense = total_expense + $6,
                updated_at = now()
            where
                user_id = $1
                and asset_type_id = $2
            returning
                id,
                user_id,
                asset_type_id,
                available_balance,
                frozen_balance,
                total_income,
                total_expense,
                is_active"#,
            account_request.user_id,
            account_request.asset_type_id.unwrap(),
            action_type
                .available_balance_change
                .calculate_change(amount),
            action_type.frozen_balance_change.calculate_change(amount),
            action_type.total_income_change.calculate_change(amount),
            action_type.total_expense_change.calculate_change(amount),
        )
        .fetch_one(&mut **tx)
        .await?;
        // 扣减`可用余额/冻结余额`时，不允许`可用余额/冻结余额`为负数
        // 增加`可用余额/冻结余额`时，允许`可用余额/冻结余额`为负数
        // 因为管理员可能直接操作数据库修改用户`可用余额/冻结余额`，所以只在扣减操作才判断
        if (action_type.available_balance_change == Change::DEC
            && account.available_balance.is_sign_negative())
            || (action_type.frozen_balance_change == Change::DEC
                && account.frozen_balance.is_sign_negative())
        {
            return Err(Error::Custom(
                StatusCode::INTERNAL_SERVER_ERROR,
                "账户余额不足".to_string(),
            ));
        }
        sqlx::query!(
            r#"insert into account_log (account_id, action_type_id, amount_available_balance, amount_frozen_balance, amount_total_income, amount_total_expense, available_balance_after, frozen_balance_after, total_income_after, total_expense_after, order_number, description)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"#,
            account.id,
            account_request.action_type_id,
            action_type.available_balance_change.calculate_change(amount),
            action_type.frozen_balance_change.calculate_change(amount),
            action_type.total_income_change.calculate_change(amount),
            action_type.total_expense_change.calculate_change(amount),
            account.available_balance,
            account.frozen_balance,
            account.total_income,
            account.total_expense,
            account_request.order_number.as_deref().unwrap_or_default(),
            account_request.description.as_deref().unwrap_or_default(),
        ).execute(&mut **tx).await?;
        Ok(())
    }

    pub async fn action(account_request: &Vec<AccountRequest>) -> AppResult<()> {
        let mut tx = database::conn().begin().await?;
        for account_request in account_request {
            Self::update_balance(&mut tx, account_request).await?;
        }
        tx.commit().await?;
        Ok(())
    }
}
