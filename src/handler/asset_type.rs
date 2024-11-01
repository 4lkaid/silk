use crate::service::asset_type::AssetType;
use amazing::AppResult;
use axum::Json;

// 资产类型列表
pub async fn list_asset_types() -> AppResult<Json<Vec<AssetType>>> {
    let asset_type = AssetType::fetch_all().await?;
    Ok(Json(asset_type))
}
