use sea_orm::entity::prelude::*;
pub use super::_entities::media_types::{ActiveModel, Model, Entity};
pub type MediaTypes = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl Model {}
impl ActiveModel {}
impl Entity {}
