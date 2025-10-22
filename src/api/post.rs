use crate::AppState;
use crate::entity::{post, thread};
use axum::Form;
use axum::extract::State;
use chrono::Utc;
use rand::Rng;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::Deserialize;
use serde_with::NoneAsEmptyString;
use serde_with::serde_as;
use tokio::join;

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

pub async fn create(State(state): State<AppState>, Form(payload): Form<CreatePost>) -> String {
    let password: String = {
        let mut rng = rand::rng();
        (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                char::from(CHARSET[idx])
            })
            .collect()
    };

    let thread = thread::Entity::find_by_id(payload.thread_id)
        .one(&state.db)
        .await
        .unwrap();
    let mut thread: thread::ActiveModel = thread.unwrap().into();
    thread.bumped_at = Set(Utc::now().naive_utc());
    let thread_future = thread.update(&state.db);

    // TODO! перед тем как выкладывать пост - проверить, если вообще такой тред
    let post = post::ActiveModel {
        thread_id: Set(payload.thread_id),
        parent_id: Set(payload.parent_id),
        title: Set(payload.title),
        text: Set(payload.text),
        password: Set(password),
        ..Default::default()
    };
    let post_future = post.insert(&state.db);

    let (post, _) = join!(post_future, thread_future);

    let msg = format!("Created post, id - {}", post.unwrap().id);
    msg
}
