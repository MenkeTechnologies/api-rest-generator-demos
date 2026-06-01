use sea_orm::entity::prelude::*;
pub use super::_entities::staffs::{ActiveModel, Model, Entity};
pub type Staffs = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl Model {}
impl ActiveModel {}
impl Entity {}
