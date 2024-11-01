mod handler;
mod route;
mod service;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let router = route::api::init();
    let _worker_guard = amazing::run(router).await?;
    Ok(())
}
