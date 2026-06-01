use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "customers",
            &[
            ("customer_id", ColType::PkAuto),
            ("first_name", ColType::String),
            ("last_name", ColType::String),
            ("company", ColType::String),
            ("address", ColType::String),
            ("city", ColType::String),
            ("state", ColType::String),
            ("country", ColType::String),
            ("postal_code", ColType::String),
            ("phone", ColType::String),
            ("fax", ColType::String),
            ("email", ColType::String),
            ("support_rep_id", ColType::Integer),
            ],
            &[],
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "customers").await
    }
}
