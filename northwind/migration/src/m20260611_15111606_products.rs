use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "products",
            &[
            ("product_id", ColType::PkAuto),
            ("product_name", ColType::String),
            ("supplier_id", ColType::Integer),
            ("category_id", ColType::Integer),
            ("quantity_per_unit", ColType::String),
            ("unit_price", ColType::Double),
            ("units_in_stock", ColType::SmallInteger),
            ("units_on_order", ColType::SmallInteger),
            ("reorder_level", ColType::SmallInteger),
            ("discontinued", ColType::Boolean),
            ],
            &[],
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "products").await
    }
}
