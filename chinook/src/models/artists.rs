use sea_orm::entity::prelude::*;
pub use super::_entities::artists::{ActiveModel, Model, Entity};
pub type Artists = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl Model {}
impl ActiveModel {}
impl Entity {}
