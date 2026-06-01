use sea_orm::entity::prelude::*;
pub use super::_entities::customer_customer_demos::{ActiveModel, Model, Entity};
pub type CustomerCustomerDemos = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl Model {}
impl ActiveModel {}
impl Entity {}
