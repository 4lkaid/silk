use crate::handler;
use amazing::middleware::{cors, request_id, request_response_logger, trace};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;

pub fn init() -> Router {
    Router::new()
        // 获取资产类型
        .route("/asset-types", get(handler::asset_type::list_asset_types))
        // 获取账户操作类型
        .route(
            "/action-types",
            get(handler::action_type::list_action_types),
        )
        // 添加资产账户
        .route("/add-account", post(handler::account::add_account))
        // 获取资产账户信息
        .route("/account-info", post(handler::account::account_info))
        // 资产账户操作
        .route("/account-action", post(handler::account::account_action))
        .layer(middleware::from_fn(
            request_response_logger::print_request_response,
        ))
        // Its recommended to use tower::ServiceBuilder to apply multiple middleware at once, instead of calling layer (or route_layer) repeatedly.
        // ServiceBuilder works by composing all layers into one such that they run top to bottom.
        // Executing middleware top to bottom is generally easier to understand and follow mentally which is one of the reasons ServiceBuilder is recommended.
        .layer(
            ServiceBuilder::new()
                .layer(request_id::set_request_id())
                .layer(request_id::propagate_request_id())
                .layer(trace::trace())
                .layer(cors::cors()),
        )
}
