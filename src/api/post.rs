use crate::entity::{post, thread};
use crate::{AppState, api};
use axum::Form;
use axum::extract::State;
use axum::response::Redirect;
use chrono::Utc;
use rand::Rng;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TransactionTrait};
use serde::Deserialize;
use serde_with::NoneAsEmptyString;
use serde_with::serde_as;

#[serde_as]
#[derive(Deserialize)]
pub struct CreatePost {
    thread_id: i32,
    #[serde_as(as = "NoneAsEmptyString")]
    parent_id: Option<i32>,
    #[serde_as(as = "NoneAsEmptyString")]
    title: Option<String>,
    text: String,
}

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789)(*&^%$#@!~";
const PASSWORD_LEN: usize = 22;

// TODO! добавить логи
pub async fn create(
    State(state): State<AppState>,
    Form(payload): Form<CreatePost>,
) -> Result<Redirect, api::Error> {
    let txn = state.db.begin().await.map_err(api::Error::DbErr)?;

    let thread = thread::Entity::find_by_id(payload.thread_id)
        .one(&txn)
        .await
        .map_err(api::Error::DbErr)?;

    let mut thread: thread::ActiveModel = thread.ok_or_else(|| api::Error::ThreadNotFound)?.into();
    let time = Utc::now().naive_utc();

    thread.bumped_at = Set(time);
    thread.update(&txn).await.map_err(api::Error::DbErr)?;

    let password = {
        let mut rng = rand::rng();
        (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                char::from(CHARSET[idx])
            })
            .collect()
    };

    let post = post::ActiveModel {
        id: Default::default(),
        thread_id: Set(payload.thread_id),
        parent_id: Set(payload.parent_id),
        title: Set(payload.title),
        text: Set(payload.text),
        created_at: Set(time),
        password: Set(password),
    };
    let post = post.insert(&txn).await.map_err(api::Error::DbErr)?;

    txn.commit().await.map_err(api::Error::DbErr)?;

    tracing::info!("Created post №{} in a thread №{}", post.id, post.thread_id);

    let redirect_url = format!("/thread/{}#post-{}", post.thread_id, post.id);
    Ok(Redirect::to(&redirect_url))
}
