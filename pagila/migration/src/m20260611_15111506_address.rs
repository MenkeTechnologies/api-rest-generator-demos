use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "address",
            &[
            ("id", ColType::PkAuto),
            ("address_id", ColType::Integer),
            ("address", ColType::String),
            ("address2", ColType::String),
            ("district", ColType::String),
            ("city_id", ColType::Integer),
            ("postal_code", ColType::String),
            ("phone", ColType::String),
            ("last_update", ColType::TimestampWithTimeZone),
            ],
            &[],
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "address").await
    }
}
