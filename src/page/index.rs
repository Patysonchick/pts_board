use crate::{AppState, api};
use askama::Template;
use axum::extract::State;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

pub async fn index(State(_state): State<AppState>) -> Result<Html<String>, api::Error> {
    // TODO! оформить как-то главную страницу, версия движка там, аптайм и т.д

    let page = IndexTemplate { name: "pts_board" }
        .render()
        .map_err(|_| api::Error::Render)?;
    Ok(Html(page))
}
