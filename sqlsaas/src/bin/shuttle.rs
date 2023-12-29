use loco_rs::boot::{create_app, StartMode};
use loco_rs::environment::resolve_from_env;
use sqlsaas::app::App;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let environment = resolve_from_env().unwrap_or_else(|| "development".to_string());
    let boot_result = create_app::<App>(StartMode::ServerOnly, &environment)
        .await
        .unwrap();

    let router = boot_result.router.unwrap();
    Ok(router.into())
}
