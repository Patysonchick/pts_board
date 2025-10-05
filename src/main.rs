mod create_post;
mod create_thread;
mod entity;

use crate::create_post::create_post;
use crate::create_thread::create_thread;
use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    let shared_state = AppState {
        db: Database::connect("postgres://admin:qwe123@127.0.0.1:5432/database") // TODO! сделать настройку подключения через .env
            .await
            .expect("Failed to connect to db"),
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello there!" }))
        .route("/create_thread", post(create_thread))
        .route("/create_post", post(create_post))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3229").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
