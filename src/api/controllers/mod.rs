use aide::axum::ApiRouter;

use super::AppState;

mod boards;

pub fn routes(state: AppState) -> ApiRouter {
    ApiRouter::new().nest_api_service("/boards", boards::routes(state.clone()))
}
