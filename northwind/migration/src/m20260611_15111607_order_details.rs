use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "order_details",
            &[
            ("id", ColType::PkAuto),
            ("order_id", ColType::Integer),
            ("product_id", ColType::Integer),
            ("unit_price", ColType::Double),
            ("quantity", ColType::SmallInteger),
            ("discount", ColType::Float),
            ],
            &[],
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "order_details").await
    }
}
