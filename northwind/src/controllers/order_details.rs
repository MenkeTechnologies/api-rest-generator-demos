#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
#[allow(unused_imports)]
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Time};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::models::_entities::order_details::{ActiveModel, Entity as OrderDetails, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub order_id: i32,
    pub product_id: i32,
    pub unit_price: f64,
    pub quantity: i16,
    pub discount: f32,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.order_id = sea_orm::ActiveValue::set(self.order_id.clone());
        item.product_id = sea_orm::ActiveValue::set(self.product_id.clone());
        item.unit_price = sea_orm::ActiveValue::set(self.unit_price.clone());
        item.quantity = sea_orm::ActiveValue::set(self.quantity.clone());
        item.discount = sea_orm::ActiveValue::set(self.discount.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    OrderDetails::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(OrderDetails::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
        let item = ActiveModel {
            order_id: sea_orm::ActiveValue::set(params.order_id.clone()),
            product_id: sea_orm::ActiveValue::set(params.product_id.clone()),
            unit_price: sea_orm::ActiveValue::set(params.unit_price.clone()),
            quantity: sea_orm::ActiveValue::set(params.quantity.clone()),
            discount: sea_orm::ActiveValue::set(params.discount.clone()),
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
        .prefix("api/order_details")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
}
