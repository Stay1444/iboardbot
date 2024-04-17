use std::sync::Arc;

use aide::{
    axum::{
        routing::{get, get_with},
        ApiRouter, IntoApiResponse,
    },
    openapi::OpenApi,
    scalar::Scalar,
};
use axum::{response::IntoResponse, Extension};

pub fn new() -> ApiRouter {
    aide::gen::infer_responses(true);
    let router: ApiRouter = ApiRouter::new()
        .api_route_with(
            "/",
            get_with(
                Scalar::new("/docs/private/api.json")
                    .with_title("IBoardBot")
                    .axum_handler(),
                |op| op.description("IBoardBot Docs"),
            ),
            |p| p,
        )
        .route("/private/api.json", get(serve_docs));

    aide::gen::infer_responses(false);

    router
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    axum::Json(api).into_response()
}
