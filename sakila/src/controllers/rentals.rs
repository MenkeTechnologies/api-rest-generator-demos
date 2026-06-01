#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
#[allow(unused_imports)]
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Time};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::models::_entities::rentals::{ActiveModel, Entity as Rentals, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub rental_date: DateTimeWithTimeZone,
    pub inventory_id: i32,
    pub customer_id: i32,
    pub return_date: DateTimeWithTimeZone,
    pub staff_id: i32,
    pub last_update: DateTimeWithTimeZone,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.rental_date = sea_orm::ActiveValue::set(self.rental_date.clone());
        item.inventory_id = sea_orm::ActiveValue::set(self.inventory_id.clone());
        item.customer_id = sea_orm::ActiveValue::set(self.customer_id.clone());
        item.return_date = sea_orm::ActiveValue::set(self.return_date.clone());
        item.staff_id = sea_orm::ActiveValue::set(self.staff_id.clone());
        item.last_update = sea_orm::ActiveValue::set(self.last_update.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    Rentals::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Rentals::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
        let item = ActiveModel {
            rental_date: sea_orm::ActiveValue::set(params.rental_date.clone()),
            inventory_id: sea_orm::ActiveValue::set(params.inventory_id.clone()),
            customer_id: sea_orm::ActiveValue::set(params.customer_id.clone()),
            return_date: sea_orm::ActiveValue::set(params.return_date.clone()),
            staff_id: sea_orm::ActiveValue::set(params.staff_id.clone()),
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
        .prefix("api/rentals")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
}
