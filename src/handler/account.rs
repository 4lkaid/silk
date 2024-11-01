use crate::service::account::{Account, AccountRequest};
use amazing::{validation::ValidatedJson, AppResult};
use axum::{http::StatusCode, Json};

// 添加账户
pub async fn add_account(
    ValidatedJson(payload): ValidatedJson<AccountRequest>,
) -> AppResult<StatusCode> {
    Account::create(&payload).await?;
    Ok(StatusCode::CREATED)
}

// 账户信息
pub async fn account_info(
    ValidatedJson(payload): ValidatedJson<AccountRequest>,
) -> AppResult<Json<Vec<Account>>> {
    let info = Account::info(&payload).await?;
    Ok(Json(info))
}

// 账户操作
// 仅涉及可用余额、冻结余额、累计收入、累计支出的变更
pub async fn account_action(
    ValidatedJson(payload): ValidatedJson<Vec<AccountRequest>>,
) -> AppResult<()> {
    Account::action(&payload).await
}
