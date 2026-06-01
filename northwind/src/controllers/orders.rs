#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
#[allow(unused_imports)]
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Time};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::models::_entities::orders::{ActiveModel, Entity as Orders, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub customer_id: String,
    pub employee_id: i32,
    pub order_date: DateTimeWithTimeZone,
    pub required_date: DateTimeWithTimeZone,
    pub shipped_date: DateTimeWithTimeZone,
    pub ship_via: i32,
    pub freight: f64,
    pub ship_name: String,
    pub ship_address: String,
    pub ship_city: String,
    pub ship_region: String,
    pub ship_postal_code: String,
    pub ship_country: String,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.customer_id = sea_orm::ActiveValue::set(self.customer_id.clone());
        item.employee_id = sea_orm::ActiveValue::set(self.employee_id.clone());
        item.order_date = sea_orm::ActiveValue::set(self.order_date.clone());
        item.required_date = sea_orm::ActiveValue::set(self.required_date.clone());
        item.shipped_date = sea_orm::ActiveValue::set(self.shipped_date.clone());
        item.ship_via = sea_orm::ActiveValue::set(self.ship_via.clone());
        item.freight = sea_orm::ActiveValue::set(self.freight.clone());
        item.ship_name = sea_orm::ActiveValue::set(self.ship_name.clone());
        item.ship_address = sea_orm::ActiveValue::set(self.ship_address.clone());
        item.ship_city = sea_orm::ActiveValue::set(self.ship_city.clone());
        item.ship_region = sea_orm::ActiveValue::set(self.ship_region.clone());
        item.ship_postal_code = sea_orm::ActiveValue::set(self.ship_postal_code.clone());
        item.ship_country = sea_orm::ActiveValue::set(self.ship_country.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    Orders::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Orders::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
        let item = ActiveModel {
            customer_id: sea_orm::ActiveValue::set(params.customer_id.clone()),
            employee_id: sea_orm::ActiveValue::set(params.employee_id.clone()),
            order_date: sea_orm::ActiveValue::set(params.order_date.clone()),
            required_date: sea_orm::ActiveValue::set(params.required_date.clone()),
            shipped_date: sea_orm::ActiveValue::set(params.shipped_date.clone()),
            ship_via: sea_orm::ActiveValue::set(params.ship_via.clone()),
            freight: sea_orm::ActiveValue::set(params.freight.clone()),
            ship_name: sea_orm::ActiveValue::set(params.ship_name.clone()),
            ship_address: sea_orm::ActiveValue::set(params.ship_address.clone()),
            ship_city: sea_orm::ActiveValue::set(params.ship_city.clone()),
            ship_region: sea_orm::ActiveValue::set(params.ship_region.clone()),
            ship_postal_code: sea_orm::ActiveValue::set(params.ship_postal_code.clone()),
            ship_country: sea_orm::ActiveValue::set(params.ship_country.clone()),
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
        .prefix("api/orders")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
}
