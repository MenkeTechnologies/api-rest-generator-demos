use sea_orm::entity::prelude::*;
pub use super::_entities::film_texts::{ActiveModel, Model, Entity};
pub type FilmTexts = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl Model {}
impl ActiveModel {}
impl Entity {}
