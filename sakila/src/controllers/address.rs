#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
#[allow(unused_imports)]
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Time};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::models::_entities::address::{ActiveModel, Entity as Address, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub address: String,
    pub address2: String,
    pub district: String,
    pub city_id: i32,
    pub postal_code: String,
    pub phone: String,
    pub last_update: DateTimeWithTimeZone,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.address = sea_orm::ActiveValue::set(self.address.clone());
        item.address2 = sea_orm::ActiveValue::set(self.address2.clone());
        item.district = sea_orm::ActiveValue::set(self.district.clone());
        item.city_id = sea_orm::ActiveValue::set(self.city_id.clone());
        item.postal_code = sea_orm::ActiveValue::set(self.postal_code.clone());
        item.phone = sea_orm::ActiveValue::set(self.phone.clone());
        item.last_update = sea_orm::ActiveValue::set(self.last_update.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    Address::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Address::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
        let item = ActiveModel {
            address: sea_orm::ActiveValue::set(params.address.clone()),
            address2: sea_orm::ActiveValue::set(params.address2.clone()),
            district: sea_orm::ActiveValue::set(params.district.clone()),
            city_id: sea_orm::ActiveValue::set(params.city_id.clone()),
            postal_code: sea_orm::ActiveValue::set(params.postal_code.clone()),
            phone: sea_orm::ActiveValue::set(params.phone.clone()),
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
        .prefix("api/address")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
}
