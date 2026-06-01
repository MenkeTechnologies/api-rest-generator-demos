#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
#[allow(unused_imports)]
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Time};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::models::_entities::employees::{ActiveModel, Entity as Employees, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub last_name: String,
    pub first_name: String,
    pub title: String,
    pub title_of_courtesy: String,
    pub birth_date: DateTimeWithTimeZone,
    pub hire_date: DateTimeWithTimeZone,
    pub address: String,
    pub city: String,
    pub region: String,
    pub postal_code: String,
    pub country: String,
    pub home_phone: String,
    pub extension: String,
    pub photo: Vec<u8>,
    pub notes: String,
    pub reports_to: i32,
    pub photo_path: String,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.last_name = sea_orm::ActiveValue::set(self.last_name.clone());
        item.first_name = sea_orm::ActiveValue::set(self.first_name.clone());
        item.title = sea_orm::ActiveValue::set(self.title.clone());
        item.title_of_courtesy = sea_orm::ActiveValue::set(self.title_of_courtesy.clone());
        item.birth_date = sea_orm::ActiveValue::set(self.birth_date.clone());
        item.hire_date = sea_orm::ActiveValue::set(self.hire_date.clone());
        item.address = sea_orm::ActiveValue::set(self.address.clone());
        item.city = sea_orm::ActiveValue::set(self.city.clone());
        item.region = sea_orm::ActiveValue::set(self.region.clone());
        item.postal_code = sea_orm::ActiveValue::set(self.postal_code.clone());
        item.country = sea_orm::ActiveValue::set(self.country.clone());
        item.home_phone = sea_orm::ActiveValue::set(self.home_phone.clone());
        item.extension = sea_orm::ActiveValue::set(self.extension.clone());
        item.photo = sea_orm::ActiveValue::set(self.photo.clone());
        item.notes = sea_orm::ActiveValue::set(self.notes.clone());
        item.reports_to = sea_orm::ActiveValue::set(self.reports_to.clone());
        item.photo_path = sea_orm::ActiveValue::set(self.photo_path.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    Employees::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Employees::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
        let item = ActiveModel {
            last_name: sea_orm::ActiveValue::set(params.last_name.clone()),
            first_name: sea_orm::ActiveValue::set(params.first_name.clone()),
            title: sea_orm::ActiveValue::set(params.title.clone()),
            title_of_courtesy: sea_orm::ActiveValue::set(params.title_of_courtesy.clone()),
            birth_date: sea_orm::ActiveValue::set(params.birth_date.clone()),
            hire_date: sea_orm::ActiveValue::set(params.hire_date.clone()),
            address: sea_orm::ActiveValue::set(params.address.clone()),
            city: sea_orm::ActiveValue::set(params.city.clone()),
            region: sea_orm::ActiveValue::set(params.region.clone()),
            postal_code: sea_orm::ActiveValue::set(params.postal_code.clone()),
            country: sea_orm::ActiveValue::set(params.country.clone()),
            home_phone: sea_orm::ActiveValue::set(params.home_phone.clone()),
            extension: sea_orm::ActiveValue::set(params.extension.clone()),
            photo: sea_orm::ActiveValue::set(params.photo.clone()),
            notes: sea_orm::ActiveValue::set(params.notes.clone()),
            reports_to: sea_orm::ActiveValue::set(params.reports_to.clone()),
            photo_path: sea_orm::ActiveValue::set(params.photo_path.clone()),
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
        .prefix("api/employees")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
}
