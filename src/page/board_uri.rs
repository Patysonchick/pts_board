use crate::entity::{board, post, thread};
use crate::{AppState, api};
use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

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

pub async fn list(
    State(state): State<AppState>,
    Path(board_uri): Path<String>,
) -> Result<Html<String>, api::Error> {
    // TODO! обязательно сделать проверку на наличие нужной доски, обработать ошибку
    let board = board::Entity::find()
        .filter(board::Column::Uri.eq(&board_uri))
        .one(&state.db)
        .await
        .map_err(api::Error::Database)?
        .ok_or_else(|| api::Error::BoardNotFound)?;

    let threads_posts = thread::Entity::find()
        .filter(thread::Column::BoardId.eq(board.id))
        .find_with_related(post::Entity)
        .order_by_asc(thread::Column::Id)
        .order_by_asc(post::Column::Id)
        .all(&state.db)
        .await
        .map_err(api::Error::Database)?;

    let threads_posts = threads_posts
        .into_iter()
        .map(|(thread, posts)| ThreadPosts { thread, posts })
        .collect();

    let page = ThreadsTemplate {
        board,
        threads_posts,
    }
    .render()
    .map_err(|_| api::Error::Render)?;
    Ok(Html(page))
}
