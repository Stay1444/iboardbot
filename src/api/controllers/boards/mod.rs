use aide::{
    axum::{
        routing::{get_with, post_with},
        ApiRouter,
    },
    transform::TransformPathItem,
};

use crate::api::AppState;

mod list;

mod jobs {
    pub mod add;
    pub mod current;
    pub mod list;
}

pub fn routes(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route_with("/", get_with(list::action, list::docs), docs)
        .api_route_with(
            "/jobs/:boardId",
            get_with(jobs::list::action, jobs::list::docs),
            docs,
        )
        .api_route_with(
            "/jobs/:boardId",
            post_with(jobs::add::action, jobs::add::docs),
            docs,
        )
        .api_route_with(
            "/jobs/:boardId/active",
            get_with(jobs::current::action, jobs::current::docs),
            docs,
        )
        .with_state(state)
}

fn docs(op: TransformPathItem) -> TransformPathItem {
    op.tag("Boards")
}
