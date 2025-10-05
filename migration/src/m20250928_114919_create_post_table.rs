use crate::m20250928_182119_create_thread_table::Thread;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_auto(Post::Id))
                    .col(integer(Post::ThreadId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post-thread_id")
                            .from(Post::Table, Post::ThreadId)
                            .to(Thread::Table, Thread::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer_null(Post::ParentId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post-parent_id")
                            .from(Post::Table, Post::ParentId)
                            .to(Post::Table, Post::Id),
                    )
                    .col(string_null(Post::Title))
                    .col(string(Post::Text))
                    .col(timestamp(Post::CreatedAt).default(Expr::current_timestamp()))
                    .col(string(Post::Password))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    ThreadId,
    ParentId,
    Title,
    Text,
    CreatedAt,
    Password,
}
