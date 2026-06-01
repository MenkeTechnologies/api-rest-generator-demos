#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
#[allow(unused_imports)]
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Time};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::models::_entities::stores::{ActiveModel, Entity as Stores, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub store_id: i32,
    pub manager_staff_id: i32,
    pub address_id: i32,
    pub last_update: DateTimeWithTimeZone,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.store_id = sea_orm::ActiveValue::set(self.store_id.clone());
        item.manager_staff_id = sea_orm::ActiveValue::set(self.manager_staff_id.clone());
        item.address_id = sea_orm::ActiveValue::set(self.address_id.clone());
        item.last_update = sea_orm::ActiveValue::set(self.last_update.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    Stores::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Stores::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
        let item = ActiveModel {
            store_id: sea_orm::ActiveValue::set(params.store_id.clone()),
            manager_staff_id: sea_orm::ActiveValue::set(params.manager_staff_id.clone()),
            address_id: sea_orm::ActiveValue::set(params.address_id.clone()),
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
        .prefix("api/stores")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
}
