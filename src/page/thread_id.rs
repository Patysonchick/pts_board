use crate::entity::{post, thread};
use crate::{AppState, api};
use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;
use sea_orm::{ColumnTrait, QueryOrder};
use sea_orm::{EntityTrait, QueryFilter};

#[derive(Template)]
#[template(path = "thread_id.html")]
struct ThreadTemplate {
    thread: thread::Model,
    posts: Vec<post::Model>,
}

pub async fn list(
    State(state): State<AppState>,
    Path(thread_id): Path<i32>,
) -> Result<Html<String>, api::Error> {
    // TODO! обязательно сделать проверку на наличие нужного треда, обработать ошибку
    let thread = thread::Entity::find_by_id(thread_id)
        .one(&state.db)
        .await
        .map_err(api::Error::Database)?
        .ok_or_else(|| api::Error::ThreadNotFound)?;

    let posts = post::Entity::find()
        .filter(post::Column::ThreadId.eq(thread_id))
        .order_by_asc(post::Column::Id)
        .all(&state.db)
        .await
        .map_err(api::Error::Database)?;

    let page = ThreadTemplate { thread, posts }
        .render()
        .map_err(|_| api::Error::Render)?;
    Ok(Html(page))
}
