#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20260611_15111600_employees;
mod m20260611_15111601_categories;
mod m20260611_15111602_customers;
mod m20260611_15111603_shippers;
mod m20260611_15111604_suppliers;
mod m20260611_15111605_orders;
mod m20260611_15111606_products;
mod m20260611_15111607_order_details;
mod m20260611_15111608_customer_customer_demos;
mod m20260611_15111609_customer_demographics;
mod m20260611_15111610_regions;
mod m20260611_15111611_territories;
mod m20260611_15111612_employee_territories;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260611_15111600_employees::Migration),
            Box::new(m20260611_15111601_categories::Migration),
            Box::new(m20260611_15111602_customers::Migration),
            Box::new(m20260611_15111603_shippers::Migration),
            Box::new(m20260611_15111604_suppliers::Migration),
            Box::new(m20260611_15111605_orders::Migration),
            Box::new(m20260611_15111606_products::Migration),
            Box::new(m20260611_15111607_order_details::Migration),
            Box::new(m20260611_15111608_customer_customer_demos::Migration),
            Box::new(m20260611_15111609_customer_demographics::Migration),
            Box::new(m20260611_15111610_regions::Migration),
            Box::new(m20260611_15111611_territories::Migration),
            Box::new(m20260611_15111612_employee_territories::Migration),
        ]
    }
}
