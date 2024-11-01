use ::redis::AsyncCommands;
use amazing::{database, redis, AppResult};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AssetType {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl AssetType {
    pub async fn fetch_all() -> AppResult<Vec<Self>> {
        let mut con = redis::conn().await?;
        let result: Option<String> = con.get("asset_type").await?;
        if let Some(result) = result {
            if let Ok(asset_type) = serde_json::from_str::<Vec<AssetType>>(&result) {
                return Ok(asset_type);
            }
        }
        let asset_type: Vec<AssetType> = sqlx::query_as!(
            AssetType,
            r#"select id, name, description from asset_type where is_active = true"#
        )
        .fetch_all(database::conn())
        .await?;
        if let Ok(asset_type) = serde_json::to_string(&asset_type) {
            let _: () = con.set_ex("asset_type", asset_type, 10).await?;
        }
        Ok(asset_type)
    }

    #[allow(dead_code)]
    pub async fn is_active(id: i32) -> bool {
        if let Ok(Some(is_exist)) = sqlx::query_scalar!(
            "select exists(select 1 from asset_type where id = $1 and is_active = true)",
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
    pub async fn get_active_ids() -> AppResult<Vec<i32>> {
        let ids = sqlx::query_scalar!(r#"select id from asset_type where is_active = true"#)
            .fetch_all(database::conn())
            .await?;
        Ok(ids)
    }
}
