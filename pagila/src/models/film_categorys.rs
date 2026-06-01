use sea_orm::entity::prelude::*;
pub use super::_entities::film_categorys::{ActiveModel, Model, Entity};
pub type FilmCategorys = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl Model {}
impl ActiveModel {}
impl Entity {}
