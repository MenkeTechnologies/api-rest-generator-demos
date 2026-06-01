use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "invoice_lines",
            &[
            ("invoice_line_id", ColType::PkAuto),
            ("invoice_id", ColType::Integer),
            ("track_id", ColType::Integer),
            ("unit_price", ColType::Double),
            ("quantity", ColType::Integer),
            ],
            &[],
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "invoice_lines").await
    }
}
