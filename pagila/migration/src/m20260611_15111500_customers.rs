use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "customers",
            &[
            ("id", ColType::PkAuto),
            ("customer_id", ColType::Integer),
            ("store_id", ColType::Integer),
            ("first_name", ColType::String),
            ("last_name", ColType::String),
            ("email", ColType::String),
            ("address_id", ColType::Integer),
            ("activebool", ColType::Boolean),
            ("create_date", ColType::Date),
            ("last_update", ColType::TimestampWithTimeZone),
            ("active", ColType::Integer),
            ],
            &[],
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "customers").await
    }
}
