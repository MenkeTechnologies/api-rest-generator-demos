#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
#[allow(unused_imports)]
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Time};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::models::_entities::payment_p2022_07s::{ActiveModel, Entity as PaymentP202207s, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub payment_id: i32,
    pub customer_id: i32,
    pub staff_id: i32,
    pub rental_id: i32,
    pub amount: f64,
    pub payment_date: DateTimeWithTimeZone,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.payment_id = sea_orm::ActiveValue::set(self.payment_id.clone());
        item.customer_id = sea_orm::ActiveValue::set(self.customer_id.clone());
        item.staff_id = sea_orm::ActiveValue::set(self.staff_id.clone());
        item.rental_id = sea_orm::ActiveValue::set(self.rental_id.clone());
        item.amount = sea_orm::ActiveValue::set(self.amount.clone());
        item.payment_date = sea_orm::ActiveValue::set(self.payment_date.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    PaymentP202207s::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(PaymentP202207s::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
        let item = ActiveModel {
            payment_id: sea_orm::ActiveValue::set(params.payment_id.clone()),
            customer_id: sea_orm::ActiveValue::set(params.customer_id.clone()),
            staff_id: sea_orm::ActiveValue::set(params.staff_id.clone()),
            rental_id: sea_orm::ActiveValue::set(params.rental_id.clone()),
            amount: sea_orm::ActiveValue::set(params.amount.clone()),
            payment_date: sea_orm::ActiveValue::set(params.payment_date.clone()),
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
        .prefix("api/payment_p2022_07s")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
}
