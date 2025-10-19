use crate::AppState;
use crate::entity::{board, post, thread};
use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[derive(Template)]
#[template(path = "board_uri.html")]
struct ThreadsTemplate {
    board: board::Model,
    threads_posts: Vec<ThreadPosts>,
}

struct ThreadPosts {
    thread: thread::Model,
    posts: Vec<post::Model>,
}

pub async fn list(State(state): State<AppState>, Path(board_uri): Path<String>) -> Html<String> {
    // TODO! обязательно сделать проверку на наличие нужной доски, обработать ошибку
    let board = board::Entity::find()
        .filter(board::Column::Uri.eq(&board_uri))
        .one(&state.db)
        .await
        .unwrap()
        .unwrap();

    let threads_posts = thread::Entity::find()
        .filter(thread::Column::BoardId.eq(board.id))
        .find_with_related(post::Entity)
        .all(&state.db)
        .await
        .unwrap();

    let threads_posts = threads_posts
        .into_iter()
        .map(|(thread, posts)| ThreadPosts { thread, posts })
        .collect();

    let template = ThreadsTemplate {
        board,
        threads_posts,
    };
    Html(template.render().unwrap())
}
