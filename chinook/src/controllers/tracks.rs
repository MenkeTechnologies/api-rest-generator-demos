#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
#[allow(unused_imports)]
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Time};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::models::_entities::tracks::{ActiveModel, Entity as Tracks, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: String,
    pub album_id: i32,
    pub media_type_id: i32,
    pub genre_id: i32,
    pub composer: String,
    pub milliseconds: i32,
    pub bytes: i32,
    pub unit_price: f64,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.name = sea_orm::ActiveValue::set(self.name.clone());
        item.album_id = sea_orm::ActiveValue::set(self.album_id.clone());
        item.media_type_id = sea_orm::ActiveValue::set(self.media_type_id.clone());
        item.genre_id = sea_orm::ActiveValue::set(self.genre_id.clone());
        item.composer = sea_orm::ActiveValue::set(self.composer.clone());
        item.milliseconds = sea_orm::ActiveValue::set(self.milliseconds.clone());
        item.bytes = sea_orm::ActiveValue::set(self.bytes.clone());
        item.unit_price = sea_orm::ActiveValue::set(self.unit_price.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    Tracks::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Tracks::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
        let item = ActiveModel {
            name: sea_orm::ActiveValue::set(params.name.clone()),
            album_id: sea_orm::ActiveValue::set(params.album_id.clone()),
            media_type_id: sea_orm::ActiveValue::set(params.media_type_id.clone()),
            genre_id: sea_orm::ActiveValue::set(params.genre_id.clone()),
            composer: sea_orm::ActiveValue::set(params.composer.clone()),
            milliseconds: sea_orm::ActiveValue::set(params.milliseconds.clone()),
            bytes: sea_orm::ActiveValue::set(params.bytes.clone()),
            unit_price: sea_orm::ActiveValue::set(params.unit_price.clone()),
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
        .prefix("api/tracks")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
}
