use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Board::Table)
                    .if_not_exists()
                    .col(pk_auto(Board::Id))
                    .col(string_uniq(Board::Uri))
                    .col(string_null(Board::Name))
                    .col(string_null(Board::Description))
                    .col(big_integer_null(Board::MaxThreads))
                    .col(date_null(Board::CreatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Board::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Board {
    Table,
    Id,
    Uri,
    Name,
    Description,
    MaxThreads,
    CreatedAt,
}
