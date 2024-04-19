use aide::transform::TransformOperation;
use axum::{
    extract::{Path, State},
    Json,
};

use crate::api::services::boards::{
    entities::{Job, JobAction},
    Boards,
};

pub async fn action(State(boards): State<Boards>, Path(id): Path<String>) -> Json<Vec<Job>> {
    let jobs = boards.list_pending_jobs(id).await;

    Json(jobs)
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.description("List the pending jobs for the specified board")
        .summary("List Board Jobs")
        .response_with::<200, Json<Vec<Job>>, _>(|res| {
            res.example(vec![Job {
                id: 21,
                action: JobAction::WriteText("Hello".into()),
            }])
        })
}
