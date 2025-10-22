mod api;
mod entity;
mod page;

use crate::page::{board_uri, boards, index::index, thread_id};
use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::{Database, DatabaseConnection};
use tower_http::services::{ServeDir, ServeFile};

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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3229").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Цели
// TODO! сделать обработку ошибок, вывод ошибок пользователю
// TODO! сделать капчу
// TODO! Написать нормальные css-ки, чтобы не выглядело вырвиглазно
// TODO! сделать README.md с документацией
// TODO! сделать текстборду имиджбордой(прикрутить к тредам/постам пикчи)
// TODO! добавить отправку с задержкой(например в начале каждой минуты-10 минут)
// TODO! Сделать генерацию страниц не каждый раз, а только при изменении страницы(появление новой доски, треда, поста)
