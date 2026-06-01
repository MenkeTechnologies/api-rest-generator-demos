#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
#[allow(unused_imports)]
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Time};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::models::_entities::products::{ActiveModel, Entity as Products, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub product_name: String,
    pub supplier_id: i32,
    pub category_id: i32,
    pub quantity_per_unit: String,
    pub unit_price: f64,
    pub units_in_stock: i16,
    pub units_on_order: i16,
    pub reorder_level: i16,
    pub discontinued: bool,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.product_name = sea_orm::ActiveValue::set(self.product_name.clone());
        item.supplier_id = sea_orm::ActiveValue::set(self.supplier_id.clone());
        item.category_id = sea_orm::ActiveValue::set(self.category_id.clone());
        item.quantity_per_unit = sea_orm::ActiveValue::set(self.quantity_per_unit.clone());
        item.unit_price = sea_orm::ActiveValue::set(self.unit_price.clone());
        item.units_in_stock = sea_orm::ActiveValue::set(self.units_in_stock.clone());
        item.units_on_order = sea_orm::ActiveValue::set(self.units_on_order.clone());
        item.reorder_level = sea_orm::ActiveValue::set(self.reorder_level.clone());
        item.discontinued = sea_orm::ActiveValue::set(self.discontinued.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    Products::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Products::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
        let item = ActiveModel {
            product_name: sea_orm::ActiveValue::set(params.product_name.clone()),
            supplier_id: sea_orm::ActiveValue::set(params.supplier_id.clone()),
            category_id: sea_orm::ActiveValue::set(params.category_id.clone()),
            quantity_per_unit: sea_orm::ActiveValue::set(params.quantity_per_unit.clone()),
            unit_price: sea_orm::ActiveValue::set(params.unit_price.clone()),
            units_in_stock: sea_orm::ActiveValue::set(params.units_in_stock.clone()),
            units_on_order: sea_orm::ActiveValue::set(params.units_on_order.clone()),
            reorder_level: sea_orm::ActiveValue::set(params.reorder_level.clone()),
            discontinued: sea_orm::ActiveValue::set(params.discontinued.clone()),
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
        .prefix("api/products")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
}
