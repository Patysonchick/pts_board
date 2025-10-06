use crate::AppState;
use crate::entity::board;
use askama::Template;
use axum::extract::State;
use axum::response::Html;
use sea_orm::{EntityTrait, QueryOrder};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

pub async fn index(State(state): State<AppState>) -> Html<String> {
    let template = IndexTemplate { name: "pts_board" };

    // TODO!

    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "boards.html")]
struct BoardsTemplate {
    boards: Vec<board::Model>,
}

pub async fn list_boards(State(state): State<AppState>) -> Html<String> {
    let boards: Vec<board::Model> = board::Entity::find()
        .order_by_asc(board::Column::Id)
        .all(&state.db)
        .await
        .unwrap();
    let template = BoardsTemplate { boards };

    Html(template.render().unwrap())
}
