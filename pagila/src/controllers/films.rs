#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
#[allow(unused_imports)]
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Time};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::models::_entities::films::{ActiveModel, Entity as Films, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub film_id: i32,
    pub title: String,
    pub description: String,
    pub language_id: i32,
    pub original_language_id: i32,
    pub rental_duration: i16,
    pub rental_rate: f64,
    pub length: i16,
    pub replacement_cost: f64,
    pub last_update: DateTimeWithTimeZone,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.film_id = sea_orm::ActiveValue::set(self.film_id.clone());
        item.title = sea_orm::ActiveValue::set(self.title.clone());
        item.description = sea_orm::ActiveValue::set(self.description.clone());
        item.language_id = sea_orm::ActiveValue::set(self.language_id.clone());
        item.original_language_id = sea_orm::ActiveValue::set(self.original_language_id.clone());
        item.rental_duration = sea_orm::ActiveValue::set(self.rental_duration.clone());
        item.rental_rate = sea_orm::ActiveValue::set(self.rental_rate.clone());
        item.length = sea_orm::ActiveValue::set(self.length.clone());
        item.replacement_cost = sea_orm::ActiveValue::set(self.replacement_cost.clone());
        item.last_update = sea_orm::ActiveValue::set(self.last_update.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    Films::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Films::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
        let item = ActiveModel {
            film_id: sea_orm::ActiveValue::set(params.film_id.clone()),
            title: sea_orm::ActiveValue::set(params.title.clone()),
            description: sea_orm::ActiveValue::set(params.description.clone()),
            language_id: sea_orm::ActiveValue::set(params.language_id.clone()),
            original_language_id: sea_orm::ActiveValue::set(params.original_language_id.clone()),
            rental_duration: sea_orm::ActiveValue::set(params.rental_duration.clone()),
            rental_rate: sea_orm::ActiveValue::set(params.rental_rate.clone()),
            length: sea_orm::ActiveValue::set(params.length.clone()),
            replacement_cost: sea_orm::ActiveValue::set(params.replacement_cost.clone()),
            last_update: sea_orm::ActiveValue::set(params.last_update.clone()),
            ..Default::default()
        };
        let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/films")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
}
