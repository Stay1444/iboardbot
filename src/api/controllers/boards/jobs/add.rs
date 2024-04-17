use aide::transform::TransformOperation;
use axum::{
    extract::{Path, State},
    Json,
};

use crate::api::services::boards::{
    entities::{Job, JobAction},
    Boards,
};

pub async fn action(
    State(boards): State<Boards>,
    Path(id): Path<String>,
    Json(action): Json<JobAction>,
) -> Json<Job> {
    let job = boards.add_job(id, action).await;

    Json(job)
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.description("Queue job for the specified board")
        .summary("Queue Job")
        .response_with::<200, Json<Job>, _>(|res| {
            res.example(Job {
                id: 62,
                action: JobAction::Write(vec!["Hello".into(), "World".into()]),
            })
        })
}
