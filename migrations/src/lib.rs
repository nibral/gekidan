pub use sea_orm_migration::prelude::*;

mod m20220713_000001_create_user_table;
mod m20220724_000001_create_user_rsa_key_table;
mod m20230801_000001_create_note_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220713_000001_create_user_table::Migration),
            Box::new(m20220724_000001_create_user_rsa_key_table::Migration),
            Box::new(m20230801_000001_create_note_table::Migration),
        ]
    }
}
