use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "orders",
            &[
            ("order_id", ColType::PkAuto),
            ("customer_id", ColType::String),
            ("employee_id", ColType::Integer),
            ("order_date", ColType::TimestampWithTimeZone),
            ("required_date", ColType::TimestampWithTimeZone),
            ("shipped_date", ColType::TimestampWithTimeZone),
            ("ship_via", ColType::Integer),
            ("freight", ColType::Double),
            ("ship_name", ColType::String),
            ("ship_address", ColType::String),
            ("ship_city", ColType::String),
            ("ship_region", ColType::String),
            ("ship_postal_code", ColType::String),
            ("ship_country", ColType::String),
            ],
            &[],
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "orders").await
    }
}
