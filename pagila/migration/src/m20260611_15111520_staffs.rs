use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "staffs",
            &[
            ("id", ColType::PkAuto),
            ("staff_id", ColType::Integer),
            ("first_name", ColType::String),
            ("last_name", ColType::String),
            ("address_id", ColType::Integer),
            ("email", ColType::String),
            ("store_id", ColType::Integer),
            ("active", ColType::Boolean),
            ("username", ColType::String),
            ("password", ColType::String),
            ("last_update", ColType::TimestampWithTimeZone),
            ],
            &[],
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "staffs").await
    }
}
