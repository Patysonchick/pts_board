pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_board_table;
mod m20250928_114919_create_post_table;
mod m20250928_182119_create_thread_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_board_table::Migration),
            Box::new(m20250928_182119_create_thread_table::Migration),
            Box::new(m20250928_114919_create_post_table::Migration),
        ]
    }
}
