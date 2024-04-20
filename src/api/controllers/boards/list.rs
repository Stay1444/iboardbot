use aide::transform::TransformOperation;
use axum::{extract::State, Json};
use chrono::Utc;
use reqwest::StatusCode;

use crate::api::{
    response::ApiResponse,
    services::boards::{
        entities::{Board, BoardDetails, BoardState},
        Boards,
    },
};

pub async fn action(State(boards): State<Boards>) -> (StatusCode, ApiResponse<Vec<Board>>) {
    let boards = boards.list().await;

    ApiResponse::ok(Some(boards))
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.description("List the active Boards")
        .summary("List Boards")
        .response_with::<200, Json<ApiResponse<Vec<Board>>>, _>(|res| {
            res.example(ApiResponse {
                success: true,
                error: None,
                data: Some(vec![Board {
                    id: "main".into(),
                    state: BoardState::Ready,
                    details: BoardDetails::default(),
                    available: (0.0, 0.0, 1000.0, 1000.0),
                    last_update: Utc::now(),
                }]),
            })
        })
}
