use crate::m20220101_000001_create_board_table::Board;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Thread::Table)
                    .if_not_exists()
                    .col(pk_auto(Thread::Id))
                    .col(integer(Thread::BoardId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-thread-board_id")
                            .from(Thread::Table, Thread::BoardId)
                            .to(Board::Table, Board::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(string_null(Thread::Title))
                    .col(string(Thread::Text))
                    .col(boolean(Thread::IsPinned))
                    .col(boolean(Thread::IsClosed))
                    .col(timestamp(Thread::CreatedAt))
                    .col(timestamp(Thread::BumpedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Thread::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Thread {
    Table,
    Id,
    BoardId,
    Title,
    Text,
    IsPinned,
    IsClosed,
    CreatedAt,
    BumpedAt,
}
