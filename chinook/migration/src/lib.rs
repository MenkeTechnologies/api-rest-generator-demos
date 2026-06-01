#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20260611_15111500_albums;
mod m20260611_15111501_artists;
mod m20260611_15111502_customers;
mod m20260611_15111503_employees;
mod m20260611_15111504_genres;
mod m20260611_15111505_invoices;
mod m20260611_15111506_invoice_lines;
mod m20260611_15111507_media_types;
mod m20260611_15111508_playlists;
mod m20260611_15111509_playlist_tracks;
mod m20260611_15111510_tracks;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260611_15111500_albums::Migration),
            Box::new(m20260611_15111501_artists::Migration),
            Box::new(m20260611_15111502_customers::Migration),
            Box::new(m20260611_15111503_employees::Migration),
            Box::new(m20260611_15111504_genres::Migration),
            Box::new(m20260611_15111505_invoices::Migration),
            Box::new(m20260611_15111506_invoice_lines::Migration),
            Box::new(m20260611_15111507_media_types::Migration),
            Box::new(m20260611_15111508_playlists::Migration),
            Box::new(m20260611_15111509_playlist_tracks::Migration),
            Box::new(m20260611_15111510_tracks::Migration),
        ]
    }
}
