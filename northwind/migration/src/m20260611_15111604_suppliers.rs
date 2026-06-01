use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "suppliers",
            &[
            ("supplier_id", ColType::PkAuto),
            ("company_name", ColType::String),
            ("contact_name", ColType::String),
            ("contact_title", ColType::String),
            ("address", ColType::String),
            ("city", ColType::String),
            ("region", ColType::String),
            ("postal_code", ColType::String),
            ("country", ColType::String),
            ("phone", ColType::String),
            ("fax", ColType::String),
            ("home_page", ColType::String),
            ],
            &[],
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "suppliers").await
    }
}
