#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20260611_15111500_customers;
mod m20260611_15111501_actors;
mod m20260611_15111502_categorys;
mod m20260611_15111503_films;
mod m20260611_15111504_film_actors;
mod m20260611_15111505_film_categorys;
mod m20260611_15111506_address;
mod m20260611_15111507_citys;
mod m20260611_15111508_countrys;
mod m20260611_15111509_inventorys;
mod m20260611_15111510_languages;
mod m20260611_15111511_payments;
mod m20260611_15111512_payment_p2022_01s;
mod m20260611_15111513_payment_p2022_02s;
mod m20260611_15111514_payment_p2022_03s;
mod m20260611_15111515_payment_p2022_04s;
mod m20260611_15111516_payment_p2022_05s;
mod m20260611_15111517_payment_p2022_06s;
mod m20260611_15111518_payment_p2022_07s;
mod m20260611_15111519_rentals;
mod m20260611_15111520_staffs;
mod m20260611_15111521_stores;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260611_15111500_customers::Migration),
            Box::new(m20260611_15111501_actors::Migration),
            Box::new(m20260611_15111502_categorys::Migration),
            Box::new(m20260611_15111503_films::Migration),
            Box::new(m20260611_15111504_film_actors::Migration),
            Box::new(m20260611_15111505_film_categorys::Migration),
            Box::new(m20260611_15111506_address::Migration),
            Box::new(m20260611_15111507_citys::Migration),
            Box::new(m20260611_15111508_countrys::Migration),
            Box::new(m20260611_15111509_inventorys::Migration),
            Box::new(m20260611_15111510_languages::Migration),
            Box::new(m20260611_15111511_payments::Migration),
            Box::new(m20260611_15111512_payment_p2022_01s::Migration),
            Box::new(m20260611_15111513_payment_p2022_02s::Migration),
            Box::new(m20260611_15111514_payment_p2022_03s::Migration),
            Box::new(m20260611_15111515_payment_p2022_04s::Migration),
            Box::new(m20260611_15111516_payment_p2022_05s::Migration),
            Box::new(m20260611_15111517_payment_p2022_06s::Migration),
            Box::new(m20260611_15111518_payment_p2022_07s::Migration),
            Box::new(m20260611_15111519_rentals::Migration),
            Box::new(m20260611_15111520_staffs::Migration),
            Box::new(m20260611_15111521_stores::Migration),
        ]
    }
}
