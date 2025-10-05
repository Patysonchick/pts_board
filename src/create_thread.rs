use crate::AppState;
use crate::entity::thread;
use axum::Json;
use axum::extract::State;
use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateThread {
    board_id: i32,
    title: Option<String>,
    text: String,
}

pub async fn create_thread(
    State(state): State<AppState>,
    Json(payload): Json<CreateThread>,
) -> String {
    let thread = thread::ActiveModel {
        board_id: Set(payload.board_id),
        title: Set(payload.title),
        text: Set(payload.text),
        is_pinned: Set(false),
        is_closed: Set(false),
        ..Default::default()
    };

    let thread: thread::Model = thread.insert(&state.db).await.unwrap();
    let msg = format!("Created thread, id - {}", thread.id);
    println!("{msg}");
    msg
}
