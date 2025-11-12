use crate::entity::board;
use crate::{AppState, api};
use askama::Template;
use axum::extract::State;
use axum::response::Html;
use sea_orm::{EntityTrait, QueryOrder};

#[derive(Template)]
#[template(path = "boards.html")]
struct BoardsTemplate {
    boards: Vec<board::Model>,
}

pub async fn list(State(state): State<AppState>) -> Result<Html<String>, api::Error> {
    let boards = board::Entity::find()
        .order_by_asc(board::Column::Id)
        .all(&state.db)
        .await
        .map_err(api::Error::Database)?;

    let page = BoardsTemplate { boards }
        .render()
        .map_err(|_| api::Error::Render)?;
    Ok(Html(page))
}
