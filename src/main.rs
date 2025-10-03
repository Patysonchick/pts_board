mod entity;

use axum::{
    Router,
    extract::State,
    routing::{get, post},
};
use chrono::prelude::*;
use entity::*;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, Set};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    let shared_state = AppState {
        db: Database::connect("postgres://admin:qwe123@127.0.0.1:5432/database")
            .await
            .expect("Failed to connect to db"),
        // TODO! сделать настройку подключения через
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello there!" }))
        .route("/create_thread", post(create_thread))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3229").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_thread(State(state): State<AppState>) {
    let time = Utc::now().naive_utc();

    let thread = thread::ActiveModel {
        id: Default::default(),
        board_id: Set(2),
        title: Set(Some("Test thread".to_owned())),
        text: Set("Test test test".to_owned()),
        is_pinned: Set(false),
        is_closed: Set(false),
        created_at: Set(time),
        bumped_at: Set(time),
    };

    let thread: thread::Model = thread.insert(&state.db).await.unwrap();
}
