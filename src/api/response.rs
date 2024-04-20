use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use schemars::JsonSchema;
use serde::Serialize;
use tracing::error;

#[derive(Serialize, Clone, Debug, JsonSchema)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn internal_server_error(log: &str) -> (StatusCode, ApiResponse<T>) {
        error!("{log}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            ApiResponse {
                success: false,
                error: Some("internal server error".into()),
                data: None,
            },
        )
    }

    pub fn option(data: Option<T>, name: &str) -> (StatusCode, ApiResponse<T>) {
        match data {
            Some(x) => ApiResponse::ok(Some(x)),
            None => ApiResponse::not_found(name),
        }
    }

    pub fn ok(data: Option<T>) -> (StatusCode, ApiResponse<T>) {
        (
            StatusCode::OK,
            ApiResponse {
                success: true,
                error: None,
                data,
            },
        )
    }

    pub fn not_found(name: &str) -> (StatusCode, ApiResponse<T>) {
        (
            StatusCode::NOT_FOUND,
            ApiResponse {
                success: false,
                error: Some(format!("{} not found", name)),
                data: None,
            },
        )
    }

    pub fn bad_req(err: impl Into<String>) -> (StatusCode, ApiResponse<T>) {
        (
            StatusCode::BAD_REQUEST,
            ApiResponse {
                success: false,
                error: Some(err.into()),
                data: None,
            },
        )
    }

    pub fn err(
        err: impl std::fmt::Display,
        log: impl Into<String>,
    ) -> (StatusCode, ApiResponse<T>) {
        error!("An error occurred whilst {}: {}", log.into(), err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            ApiResponse {
                success: false,
                data: None,
                error: Some("internal server error".into()),
            },
        )
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let json = axum::Json(self);
        json.into_response()
    }
}
