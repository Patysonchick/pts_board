use crate::AppState;
use crate::entity::{post, thread};
use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;
use sea_orm::ColumnTrait;
use sea_orm::{EntityTrait, QueryFilter};

#[derive(Template)]
#[template(path = "thread_id.html")]
struct ThreadTemplate {
    thread: thread::Model,
    posts: Vec<post::Model>,
}

pub async fn list(State(state): State<AppState>, Path(thread_id): Path<i32>) -> Html<String> {
    // TODO! обязательно сделать проверку на наличие нужного треда, обработать ошибку
    let thread = thread::Entity::find_by_id(thread_id)
        .one(&state.db)
        .await
        .unwrap()
        .unwrap();

    let posts = post::Entity::find()
        .filter(post::Column::ThreadId.eq(thread_id))
        .all(&state.db)
        .await
        .unwrap();

    let template = ThreadTemplate { thread, posts };
    Html(template.render().unwrap())
}
