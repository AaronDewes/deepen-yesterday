use sea_orm::DatabaseConnection;
use sea_orm_migration::prelude::*;

pub mod m_20250330_000001_init;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m_20250330_000001_init::Migration)]
    }
}

pub async fn run(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db, None).await
}
