#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
#[allow(unused_imports)]
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Time};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::models::_entities::actors::{ActiveModel, Entity as Actors, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub actor_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub last_update: DateTimeWithTimeZone,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.actor_id = sea_orm::ActiveValue::set(self.actor_id.clone());
        item.first_name = sea_orm::ActiveValue::set(self.first_name.clone());
        item.last_name = sea_orm::ActiveValue::set(self.last_name.clone());
        item.last_update = sea_orm::ActiveValue::set(self.last_update.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    Actors::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Actors::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
        let item = ActiveModel {
            actor_id: sea_orm::ActiveValue::set(params.actor_id.clone()),
            first_name: sea_orm::ActiveValue::set(params.first_name.clone()),
            last_name: sea_orm::ActiveValue::set(params.last_name.clone()),
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
        .prefix("api/actors")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
}
