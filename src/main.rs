mod api;
mod entity;
mod page;

use crate::page::{board_uri, boards, index::index, thread_id};
use axum::{
    Router,
    routing::{get, post},
};
use migration::{Migrator, MigratorTrait};
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
    match dotenvy::dotenv() {
        Ok(_) => tracing::info!("Found .env file"),
        Err(e) => tracing::warn!("{e}\nFailed reading .env file, using default env vars"),
    }

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    tracing::info!("Starting pts_board! )");

    let db = match env::var("DATABASE_URL") {
        Ok(db) => db,
        Err(e) => {
            tracing::error!("DATABASE_URL not found");
            return Err(e.into());
        }
    };
    let db = Database::connect(db).await?;
    tracing::info!("Database connection established");

    tracing::info!("Checking for new migrations...");
    Migrator::up(&db, None).await?;
    tracing::info!("Migrations applied successfully!");

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

    let listen = match env::var("LISTEN") {
        Ok(listen) => listen,
        Err(e) => {
            tracing::error!("LISTEN not found");
            return Err(e.into());
        }
    };
    let port = match env::var("PORT") {
        Ok(port) => port,
        Err(e) => {
            tracing::error!("PORT not found");
            return Err(e.into());
        }
    };
    let port = match port.parse::<u16>() {
        Ok(port) => port,
        Err(e) => {
            tracing::error!("PORT is not number");
            return Err(e.into());
        }
    };
    tracing::info!("Server listening on {}:{}", listen, port);

    let listener = tokio::net::TcpListener::bind((listen, port)).await?;
    axum::serve(listener, app).await?;

    tracing::info!("Server shutdown complete");
    Ok(())
}

// Цели
// TODO! сделать обработку ошибок, вывод ошибок пользователю
// TODO! сделать обозначения номера тредов и постов
// TODO! сделать капчу
// TODO! сделать логирование лучше
// TODO! разобраться со span(tracing)
// TODO! Написать нормальные css-ки, чтобы не выглядело вырвиглазно
// TODO! сделать README.md с документацией
// TODO! сделать текстборду имиджбордой(прикрутить к тредам/постам пикчи)
// TODO! добавить отправку с задержкой(например в начале каждой минуты-10 минут)
// TODO! Сделать генерацию страниц не каждый раз, а только при изменении страницы(появление новой доски, треда, поста)
// TODO! сделать Docker образ для развёртывания
