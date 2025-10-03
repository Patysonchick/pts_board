use axum::{Router, routing::get};
use sea_orm::{Database, DatabaseConnection};

#[tokio::main]
async fn main() {
    let db: DatabaseConnection =
        Database::connect("postgres://admin:qwe123@127.0.0.1:5432/database")
            .await
            .expect("Failed to connect to db");

    let app = Router::new().route("/", get(|| async { "Hello there!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
