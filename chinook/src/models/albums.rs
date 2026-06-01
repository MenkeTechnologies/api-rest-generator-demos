use sea_orm::entity::prelude::*;
pub use super::_entities::albums::{ActiveModel, Model, Entity};
pub type Albums = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl Model {}
impl ActiveModel {}
impl Entity {}
