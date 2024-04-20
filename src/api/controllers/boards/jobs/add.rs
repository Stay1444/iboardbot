use aide::transform::TransformOperation;
use axum::{
    extract::{Multipart, Path, State},
    Json,
};

use crate::api::services::boards::{
    entities::{Job, JobAction, SVGSource, WriteText},
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
    while let Some(field) = multipart.next_field().await.unwrap_or_default() {
        let name = field.name().unwrap_or_default().to_string();
        let data = field.text().await.unwrap_or_default();

        if name == "svg" {
            let job = boards
                .add_job(id, JobAction::DrawSVG(SVGSource::Raw(data)))
                .await;

            return Json(job);
        }
    }

    return Json(Job {
        id: 0,
        action: JobAction::WriteText(WriteText {
            text: "You did something wrong dude".into(),
            font: None,
        }),
    });
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.description("Queue job for the specified board")
        .summary("Queue Job")
        .response_with::<200, Json<Job>, _>(|res| {
            res.example(Job {
                id: 62,
                action: JobAction::WriteText(WriteText {
                    text: "Hello".into(),
                    font: Some("Roboto.ttf".into()),
                }),
            })
        })
}
