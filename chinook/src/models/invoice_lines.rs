use sea_orm::entity::prelude::*;
pub use super::_entities::invoice_lines::{ActiveModel, Model, Entity};
pub type InvoiceLines = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl Model {}
impl ActiveModel {}
impl Entity {}
