use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "films",
            &[
            ("id", ColType::PkAuto),
            ("film_id", ColType::Integer),
            ("title", ColType::String),
            ("description", ColType::String),
            ("language_id", ColType::Integer),
            ("original_language_id", ColType::Integer),
            ("rental_duration", ColType::SmallInteger),
            ("rental_rate", ColType::Double),
            ("length", ColType::SmallInteger),
            ("replacement_cost", ColType::Double),
            ("last_update", ColType::TimestampWithTimeZone),
            ],
            &[],
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "films").await
    }
}
