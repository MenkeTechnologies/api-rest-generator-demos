use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "employees",
            &[
            ("employee_id", ColType::PkAuto),
            ("last_name", ColType::String),
            ("first_name", ColType::String),
            ("title", ColType::String),
            ("title_of_courtesy", ColType::String),
            ("birth_date", ColType::TimestampWithTimeZone),
            ("hire_date", ColType::TimestampWithTimeZone),
            ("address", ColType::String),
            ("city", ColType::String),
            ("region", ColType::String),
            ("postal_code", ColType::String),
            ("country", ColType::String),
            ("home_phone", ColType::String),
            ("extension", ColType::String),
            ("photo", ColType::Blob),
            ("notes", ColType::String),
            ("reports_to", ColType::Integer),
            ("photo_path", ColType::String),
            ],
            &[],
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "employees").await
    }
}
