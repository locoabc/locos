#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use crate::models::_entities::articles;

pub async fn echo(req_body: String) -> String {
    req_body
}

pub async fn list(State(ctx): State<AppContext>) -> Result<Json<Vec<articles::Model>>> {
    let res = articles::Entity::find().all(&ctx.db).await?;
    format::json(res)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("articles")
        .add("/", get(list))
}
