use crate::AppState;
use askama::Template;
use axum::extract::State;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

pub async fn index(State(_state): State<AppState>) -> Html<String> {
    let template = IndexTemplate { name: "pts_board" };

    // TODO! оформить как-то главную страницу, версия движка там, аптайм и т.д

    Html(template.render().unwrap())
}
