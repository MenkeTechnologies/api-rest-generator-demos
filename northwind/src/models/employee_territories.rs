use sea_orm::entity::prelude::*;
pub use super::_entities::employee_territories::{ActiveModel, Model, Entity};
pub type EmployeeTerritories = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl Model {}
impl ActiveModel {}
impl Entity {}
