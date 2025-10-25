use crate::entity::{post, thread};
use crate::{AppState, api};
use axum::Form;
use axum::extract::State;
use chrono::Utc;
use rand::Rng;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, Set, TransactionError, TransactionTrait};
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

// TODO! добавиить логи
pub async fn create(
    State(state): State<AppState>,
    Form(payload): Form<CreatePost>,
) -> Result<String, api::Error> {
    // TODO! сделать редирект
    match state
        .db
        .transaction(|txn| {
            Box::pin(async move {
                let thread = thread::Entity::find_by_id(payload.thread_id)
                    .one(txn)
                    .await?;

                if thread == None {
                    return Ok::<Result<post::Model, api::Error>, TransactionError<DbErr>>(Err(
                        api::Error::ThreadNotFound,
                    ));
                }

                let mut thread: thread::ActiveModel = thread.unwrap().into();
                thread.bumped_at = Set(Utc::now().naive_utc());
                thread.update(txn).await?;

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
                Ok(Ok(post.insert(txn).await?))
            })
        })
        .await
    {
        Ok(post) => match post {
            Ok(post) => {
                let msg = format!("Created post, id - {}", post.id);
                Ok(msg)
            }
            Err(err) => Err(err),
        },
        // TODO! сделать проверку на ошибки БД
        Err(_err) => Err(api::Error::DbErr), // TODO! сделать возвращения в enum ошибки DbErr
    }
    // TODO! сделать редирект
}
