use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "invoices",
            &[
            ("invoice_id", ColType::PkAuto),
            ("customer_id", ColType::Integer),
            ("invoice_date", ColType::TimestampWithTimeZone),
            ("billing_address", ColType::String),
            ("billing_city", ColType::String),
            ("billing_state", ColType::String),
            ("billing_country", ColType::String),
            ("billing_postal_code", ColType::String),
            ("total", ColType::Double),
            ],
            &[],
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "invoices").await
    }
}
