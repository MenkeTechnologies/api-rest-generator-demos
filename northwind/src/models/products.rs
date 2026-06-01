use sea_orm::entity::prelude::*;
pub use super::_entities::products::{ActiveModel, Model, Entity};
pub type Products = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl Model {}
impl ActiveModel {}
impl Entity {}
