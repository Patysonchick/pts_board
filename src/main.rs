mod create;
mod entity;
mod index;
mod list;

use crate::create::{post::create_post, thread::create_thread};
use crate::index::index;
use crate::list::{boards, threads};
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
        .route("/", get(index))
        .route("/boards", get(boards::list))
        .route("/{board_uri}", get(threads::list))
        .route("/create_thread", post(create_thread))
        .route("/create_post", post(create_post))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3229").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Цели
// TODO! добавить отправку ПОСТОВ
// TODO! сделать капчу
// TODO! сделать текстборду имиджбордой(прикрутить к тредам/постам пикчи)
// TODO! Сделать генерацию страниц не каждый раз, а только при изменении страницы(появление новой доски, треда, поста)
// TODO! Написать нормальные css-ки, чтобы не выглядело вырвиглазно
