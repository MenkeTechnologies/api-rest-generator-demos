use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "tracks",
            &[
            ("track_id", ColType::PkAuto),
            ("name", ColType::String),
            ("album_id", ColType::Integer),
            ("media_type_id", ColType::Integer),
            ("genre_id", ColType::Integer),
            ("composer", ColType::String),
            ("milliseconds", ColType::Integer),
            ("bytes", ColType::Integer),
            ("unit_price", ColType::Double),
            ],
            &[],
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "tracks").await
    }
}
