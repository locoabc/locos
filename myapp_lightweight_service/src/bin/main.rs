use loco_rs::cli;
use myapp_lightweight_service::app::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    cli::main::<App>().await
}
