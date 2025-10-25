mod api;
mod entity;
mod page;

use crate::page::{board_uri, boards, index::index, thread_id};
use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let db = Database::connect(env::var("DATABASE_URL")?).await?;
    tracing::info!("Database connection established");

    let shared_state = AppState { db };

    let api_routes = Router::new()
        .route("/create_thread", post(api::thread::create))
        .route("/create_post", post(api::post::create));

    let app = Router::new()
        .route("/", get(index))
        .route("/boards", get(boards::list))
        .route("/{board_uri}", get(board_uri::list))
        .route("/thread/{thread_id}", get(thread_id::list))
        .nest("/api", api_routes)
        .nest_service("/static", ServeDir::new("static"))
        .route_service("/favicon.ico", ServeFile::new("static/favicon.ico"))
        .with_state(shared_state);

    let address = env::var("ADDRESS")?;
    tracing::info!("Server listening on {}", address);

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    tracing::info!("Server shutdown complete");
    Ok(())
}

// Цели
// TODO! сделать обработку ошибок, вывод ошибок пользователю
// TODO! сделать капчу
// TODO! сделать логирование лучше
// TODO! разобраться со span(tracing)
// TODO! Написать нормальные css-ки, чтобы не выглядело вырвиглазно
// TODO! сделать README.md с документацией
// TODO! сделать текстборду имиджбордой(прикрутить к тредам/постам пикчи)
// TODO! добавить отправку с задержкой(например в начале каждой минуты-10 минут)
// TODO! Сделать генерацию страниц не каждый раз, а только при изменении страницы(появление новой доски, треда, поста)
