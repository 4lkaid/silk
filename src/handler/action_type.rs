use crate::service::action_type::ActionType;
use amazing::AppResult;
use axum::Json;

// 账户操作类型列表
pub async fn list_action_types() -> AppResult<Json<Vec<ActionType>>> {
    let action_type = ActionType::fetch_all().await?;
    Ok(Json(action_type))
}
