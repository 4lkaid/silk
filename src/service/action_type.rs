use ::redis::AsyncCommands;
use amazing::{database, redis, AppResult};
use num_traits::cast::FromPrimitive;
use serde::{Deserialize, Serialize};
use sqlx::types::Decimal;

#[derive(Deserialize, Serialize, sqlx::Type, PartialEq)]
pub enum Change {
    INC,
    DEC,
    NONE,
}

impl Change {
    pub fn calculate_change(&self, amount: f64) -> Decimal {
        let decimal_amount = Decimal::from_f64(amount.abs()).unwrap().trunc_with_scale(6);
        match self {
            Change::INC => decimal_amount,
            Change::DEC => -decimal_amount,
            Change::NONE => Decimal::ZERO,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ActionType {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub available_balance_change: Change,
    pub frozen_balance_change: Change,
    pub total_income_change: Change,
    pub total_expense_change: Change,
}

impl ActionType {
    pub async fn fetch_all() -> AppResult<Vec<Self>> {
        let mut con = redis::conn().await?;
        let result: Option<String> = con.get("action_type").await?;
        if let Some(result) = result {
            if let Ok(action_type) = serde_json::from_str(&result) {
                return Ok(action_type);
            }
        }
        let action_type: Vec<ActionType> = sqlx::query_as!(
            ActionType,
            r#"select
                id,
                name,
                description,
                available_balance_change as "available_balance_change!: Change",
                frozen_balance_change as "frozen_balance_change!: Change",
                total_income_change as "total_income_change!: Change",
                total_expense_change as "total_expense_change!: Change"
            from
                action_type
            where
                is_active = true"#
        )
        .fetch_all(database::conn())
        .await?;
        if let Ok(action_type) = serde_json::to_string(&action_type) {
            let _: () = con.set_ex("action_type", action_type, 10).await?;
        }
        Ok(action_type)
    }

    #[allow(dead_code)]
    pub async fn is_active(id: i32) -> bool {
        if let Ok(Some(is_exist)) = sqlx::query_scalar!(
            "select exists(select 1 from action_type where id = $1 and is_active = true)",
            id
        )
        .fetch_one(database::conn())
        .await
        {
            return is_exist;
        }
        false
    }

    #[allow(dead_code)]
    pub async fn fetch_one(id: i32) -> AppResult<Self> {
        let action_type = sqlx::query_as!(
            ActionType,
            r#"select
                id,
                name,
                description,
                available_balance_change as "available_balance_change!: Change",
                frozen_balance_change as "frozen_balance_change!: Change",
                total_income_change as "total_income_change!: Change",
                total_expense_change as "total_expense_change!: Change"
            from
                action_type
            where
                id = $1 and is_active = true"#,
            id
        )
        .fetch_one(database::conn())
        .await?;
        Ok(action_type)
    }
}
