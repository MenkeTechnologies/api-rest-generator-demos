#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20260611_15111400_actors;
mod m20260611_15111401_address;
mod m20260611_15111402_categorys;
mod m20260611_15111403_citys;
mod m20260611_15111404_countrys;
mod m20260611_15111405_customers;
mod m20260611_15111406_films;
mod m20260611_15111407_film_actors;
mod m20260611_15111408_film_categorys;
mod m20260611_15111409_film_texts;
mod m20260611_15111410_inventorys;
mod m20260611_15111411_languages;
mod m20260611_15111412_payments;
mod m20260611_15111413_rentals;
mod m20260611_15111414_staffs;
mod m20260611_15111415_stores;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260611_15111400_actors::Migration),
            Box::new(m20260611_15111401_address::Migration),
            Box::new(m20260611_15111402_categorys::Migration),
            Box::new(m20260611_15111403_citys::Migration),
            Box::new(m20260611_15111404_countrys::Migration),
            Box::new(m20260611_15111405_customers::Migration),
            Box::new(m20260611_15111406_films::Migration),
            Box::new(m20260611_15111407_film_actors::Migration),
            Box::new(m20260611_15111408_film_categorys::Migration),
            Box::new(m20260611_15111409_film_texts::Migration),
            Box::new(m20260611_15111410_inventorys::Migration),
            Box::new(m20260611_15111411_languages::Migration),
            Box::new(m20260611_15111412_payments::Migration),
            Box::new(m20260611_15111413_rentals::Migration),
            Box::new(m20260611_15111414_staffs::Migration),
            Box::new(m20260611_15111415_stores::Migration),
        ]
    }
}
