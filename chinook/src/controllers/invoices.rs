#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
#[allow(unused_imports)]
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Time};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::models::_entities::invoices::{ActiveModel, Entity as Invoices, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub customer_id: i32,
    pub invoice_date: DateTimeWithTimeZone,
    pub billing_address: String,
    pub billing_city: String,
    pub billing_state: String,
    pub billing_country: String,
    pub billing_postal_code: String,
    pub total: f64,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.customer_id = sea_orm::ActiveValue::set(self.customer_id.clone());
        item.invoice_date = sea_orm::ActiveValue::set(self.invoice_date.clone());
        item.billing_address = sea_orm::ActiveValue::set(self.billing_address.clone());
        item.billing_city = sea_orm::ActiveValue::set(self.billing_city.clone());
        item.billing_state = sea_orm::ActiveValue::set(self.billing_state.clone());
        item.billing_country = sea_orm::ActiveValue::set(self.billing_country.clone());
        item.billing_postal_code = sea_orm::ActiveValue::set(self.billing_postal_code.clone());
        item.total = sea_orm::ActiveValue::set(self.total.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    Invoices::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Invoices::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
        let item = ActiveModel {
            customer_id: sea_orm::ActiveValue::set(params.customer_id.clone()),
            invoice_date: sea_orm::ActiveValue::set(params.invoice_date.clone()),
            billing_address: sea_orm::ActiveValue::set(params.billing_address.clone()),
            billing_city: sea_orm::ActiveValue::set(params.billing_city.clone()),
            billing_state: sea_orm::ActiveValue::set(params.billing_state.clone()),
            billing_country: sea_orm::ActiveValue::set(params.billing_country.clone()),
            billing_postal_code: sea_orm::ActiveValue::set(params.billing_postal_code.clone()),
            total: sea_orm::ActiveValue::set(params.total.clone()),
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
        .prefix("api/invoices")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
}
