use crate::entity::thread;
use crate::{AppState, api};
use axum::Form;
use axum::extract::State;
use axum::response::Redirect;
use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;
use serde_with::NoneAsEmptyString;
use serde_with::serde_as;

#[serde_as]
#[derive(Deserialize)]
pub struct CreateThread {
    board_id: i32,
    #[serde_as(as = "NoneAsEmptyString")]
    title: Option<String>,
    text: String,
}

// TODO! добавить логи
pub async fn create(
    State(state): State<AppState>,
    Form(payload): Form<CreateThread>,
) -> Result<Redirect, api::Error> {
    let thread = thread::ActiveModel {
        board_id: Set(payload.board_id),
        title: Set(payload.title),
        text: Set(payload.text),
        is_pinned: Set(false),
        is_closed: Set(false),
        ..Default::default()
    };

    let thread = thread.insert(&state.db).await.map_err(api::Error::DbErr)?;
    tracing::info!("Created thread №{}", thread.id);

    let redirect_url = format!("/thread/{}", thread.id);
    Ok(Redirect::to(&redirect_url))
}
