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
            ("reports_to", ColType::Integer),
            ("birth_date", ColType::TimestampWithTimeZone),
            ("hire_date", ColType::TimestampWithTimeZone),
            ("address", ColType::String),
            ("city", ColType::String),
            ("state", ColType::String),
            ("country", ColType::String),
            ("postal_code", ColType::String),
            ("phone", ColType::String),
            ("fax", ColType::String),
            ("email", ColType::String),
            ],
            &[],
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "employees").await
    }
}
