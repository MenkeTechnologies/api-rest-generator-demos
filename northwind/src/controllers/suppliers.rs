#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
#[allow(unused_imports)]
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Time};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::models::_entities::suppliers::{ActiveModel, Entity as Suppliers, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub company_name: String,
    pub contact_name: String,
    pub contact_title: String,
    pub address: String,
    pub city: String,
    pub region: String,
    pub postal_code: String,
    pub country: String,
    pub phone: String,
    pub fax: String,
    pub home_page: String,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.company_name = sea_orm::ActiveValue::set(self.company_name.clone());
        item.contact_name = sea_orm::ActiveValue::set(self.contact_name.clone());
        item.contact_title = sea_orm::ActiveValue::set(self.contact_title.clone());
        item.address = sea_orm::ActiveValue::set(self.address.clone());
        item.city = sea_orm::ActiveValue::set(self.city.clone());
        item.region = sea_orm::ActiveValue::set(self.region.clone());
        item.postal_code = sea_orm::ActiveValue::set(self.postal_code.clone());
        item.country = sea_orm::ActiveValue::set(self.country.clone());
        item.phone = sea_orm::ActiveValue::set(self.phone.clone());
        item.fax = sea_orm::ActiveValue::set(self.fax.clone());
        item.home_page = sea_orm::ActiveValue::set(self.home_page.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    Suppliers::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Suppliers::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
        let item = ActiveModel {
            company_name: sea_orm::ActiveValue::set(params.company_name.clone()),
            contact_name: sea_orm::ActiveValue::set(params.contact_name.clone()),
            contact_title: sea_orm::ActiveValue::set(params.contact_title.clone()),
            address: sea_orm::ActiveValue::set(params.address.clone()),
            city: sea_orm::ActiveValue::set(params.city.clone()),
            region: sea_orm::ActiveValue::set(params.region.clone()),
            postal_code: sea_orm::ActiveValue::set(params.postal_code.clone()),
            country: sea_orm::ActiveValue::set(params.country.clone()),
            phone: sea_orm::ActiveValue::set(params.phone.clone()),
            fax: sea_orm::ActiveValue::set(params.fax.clone()),
            home_page: sea_orm::ActiveValue::set(params.home_page.clone()),
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
        .prefix("api/suppliers")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
}
