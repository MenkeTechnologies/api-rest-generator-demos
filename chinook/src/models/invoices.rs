use sea_orm::entity::prelude::*;
pub use super::_entities::invoices::{ActiveModel, Model, Entity};
pub type Invoices = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl Model {}
impl ActiveModel {}
impl Entity {}
