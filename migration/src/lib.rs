pub use sea_orm_migration::prelude::*;

mod m20221206_205815_date_triggers;
mod m20221269_206922_user_table;
mod trigger;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221206_205815_date_triggers::Migration),
            Box::new(m20221269_206922_user_table::Migration),
        ]
    }
}
