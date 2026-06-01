use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "payments",
            &[
            ("payment_id", ColType::PkAuto),
            ("customer_id", ColType::Integer),
            ("staff_id", ColType::Integer),
            ("rental_id", ColType::Integer),
            ("amount", ColType::Double),
            ("payment_date", ColType::TimestampWithTimeZone),
            ("last_update", ColType::TimestampWithTimeZone),
            ],
            &[],
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "payments").await
    }
}
