use sea_orm::entity::prelude::*;
pub use super::_entities::order_details::{ActiveModel, Model, Entity};
pub type OrderDetails = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl Model {}
impl ActiveModel {}
impl Entity {}
