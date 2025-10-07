use crate::AppState;
use crate::entity::{board, thread};
use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[derive(Template)] // this will generate the code...
#[template(path = "threads.html")]
struct ThreadsTemplate {
    board_uri: String,
    board_name: Option<String>,
    threads: Vec<thread::Model>,
}

pub async fn list(State(state): State<AppState>, Path(board_uri): Path<String>) -> Html<String> {
    // TODO! обязательно сделать проверку на наличие нужной доски, обработать ошибку
    let board: board::Model = board::Entity::find()
        .filter(board::Column::Uri.eq(&board_uri))
        .one(&state.db)
        .await
        .unwrap()
        .unwrap();

    println!("Found board id - {}", board.id);

    // TODO! на всякий проверить что будет с пустым массивом
    let threads: Vec<thread::Model> = thread::Entity::find()
        .filter(thread::Column::BoardId.eq(board.id))
        .all(&state.db)
        .await
        .unwrap();

    let template = ThreadsTemplate {
        board_uri,
        board_name: board.name,
        threads,
    };
    Html(template.render().unwrap())
}
