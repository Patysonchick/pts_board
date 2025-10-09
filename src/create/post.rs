use crate::AppState;
use crate::entity::post;
use axum::Form;
use axum::extract::State;
use rand::Rng;
use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePost {
    thread_id: i32,
    parent_id: Option<i32>,
    title: Option<String>,
    text: String,
}

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789)(*&^%$#@!~";
const PASSWORD_LEN: usize = 22;

pub async fn create_post(State(state): State<AppState>, Form(payload): Form<CreatePost>) -> String {
    let password: String = {
        let mut rng = rand::rng();

        (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                char::from(CHARSET[idx])
            })
            .collect()
    };

    let post = post::ActiveModel {
        thread_id: Set(payload.thread_id),
        parent_id: Set(payload.parent_id),
        title: Set(payload.title),
        text: Set(payload.text),
        password: Set(password),
        ..Default::default()
    };

    let post: post::Model = post.insert(&state.db).await.unwrap();
    let msg = format!("Created post, id - {}", post.id);
    msg
}
