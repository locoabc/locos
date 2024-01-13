// src/controllers/home2.rs
use loco_rs::prelude::*;

// _ctx contains your database connection, as well as other app resource that you'll need
async fn hello(State(_ctx): State<AppContext>) -> Result<String> {
    format::text("ola, mundo")
}

pub fn routes() -> Routes {
    Routes::new().prefix("home2").add("/hello", get(hello))
}