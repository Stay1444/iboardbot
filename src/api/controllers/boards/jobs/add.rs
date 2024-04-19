use aide::transform::TransformOperation;
use axum::{
    extract::{Multipart, Path, State},
    Json,
};

use crate::api::services::boards::{
    entities::{Job, JobAction, SVGSource},
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

pub async fn action_multipart(
    State(boards): State<Boards>,
    Path(id): Path<String>,
    mut multipart: Multipart,
) -> Json<Job> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.text().await.unwrap();

        if name == "svg" {
            let job = boards
                .add_job(id, JobAction::DrawSVG(SVGSource::Raw(data)))
                .await;

            return Json(job);
        }
    }

    return Json(Job {
        id: 0,
        action: JobAction::WriteText("You did something wrong dude".into()),
    });
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.description("Queue job for the specified board")
        .summary("Queue Job")
        .response_with::<200, Json<Job>, _>(|res| {
            res.example(Job {
                id: 62,
                action: JobAction::WriteText("Hello".into()),
            })
        })
}
