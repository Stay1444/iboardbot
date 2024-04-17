use aide::transform::TransformOperation;
use axum::{extract::State, Json};

use crate::api::services::boards::{
    entities::{Board, BoardDetails, BoardState},
    Boards,
};

pub async fn action(State(boards): State<Boards>) -> Json<Vec<Board>> {
    let boards = boards.list().await;

    Json(boards)
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.description("List the active Boards")
        .summary("List Boards")
        .response_with::<200, Json<Vec<Board>>, _>(|res| {
            res.example(vec![Board {
                id: "ABC-MAIN".into(),
                state: BoardState::Ready,
                details: BoardDetails::default(),
            }])
        })
}
