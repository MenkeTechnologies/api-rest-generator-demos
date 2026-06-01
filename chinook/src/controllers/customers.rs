#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
#[allow(unused_imports)]
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Time};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::models::_entities::customers::{ActiveModel, Entity as Customers, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub first_name: String,
    pub last_name: String,
    pub company: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
    pub phone: String,
    pub fax: String,
    pub email: String,
    pub support_rep_id: i32,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.first_name = sea_orm::ActiveValue::set(self.first_name.clone());
        item.last_name = sea_orm::ActiveValue::set(self.last_name.clone());
        item.company = sea_orm::ActiveValue::set(self.company.clone());
        item.address = sea_orm::ActiveValue::set(self.address.clone());
        item.city = sea_orm::ActiveValue::set(self.city.clone());
        item.state = sea_orm::ActiveValue::set(self.state.clone());
        item.country = sea_orm::ActiveValue::set(self.country.clone());
        item.postal_code = sea_orm::ActiveValue::set(self.postal_code.clone());
        item.phone = sea_orm::ActiveValue::set(self.phone.clone());
        item.fax = sea_orm::ActiveValue::set(self.fax.clone());
        item.email = sea_orm::ActiveValue::set(self.email.clone());
        item.support_rep_id = sea_orm::ActiveValue::set(self.support_rep_id.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    Customers::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Customers::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
        let item = ActiveModel {
            first_name: sea_orm::ActiveValue::set(params.first_name.clone()),
            last_name: sea_orm::ActiveValue::set(params.last_name.clone()),
            company: sea_orm::ActiveValue::set(params.company.clone()),
            address: sea_orm::ActiveValue::set(params.address.clone()),
            city: sea_orm::ActiveValue::set(params.city.clone()),
            state: sea_orm::ActiveValue::set(params.state.clone()),
            country: sea_orm::ActiveValue::set(params.country.clone()),
            postal_code: sea_orm::ActiveValue::set(params.postal_code.clone()),
            phone: sea_orm::ActiveValue::set(params.phone.clone()),
            fax: sea_orm::ActiveValue::set(params.fax.clone()),
            email: sea_orm::ActiveValue::set(params.email.clone()),
            support_rep_id: sea_orm::ActiveValue::set(params.support_rep_id.clone()),
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
        .prefix("api/customers")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
}
