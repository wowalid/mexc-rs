use dotenv::dotenv;
use mexc_rs::futures::{MexcFuturesApiClientWithAuthentication, MexcFuturesApiEndpoint};
use mexc_rs::futures::v1::endpoints::get_account_assets::GetAccountAssets;
use mexc_rs::futures::v1::endpoints::get_server_time::GetServerTime;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "mexc_rs=debug,futures_get_account_assets=trace");
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let api_key = std::env::var("MEXC_API_KEY").expect("MEXC_API_KEY not set");
    let secret_key = std::env::var("MEXC_SECRET_KEY").expect("MEXC_SECRET_KEY not set");

    let client = MexcFuturesApiClientWithAuthentication::new(MexcFuturesApiEndpoint::Base, api_key, secret_key);
    let account_assets = client.get_account_assets().await?;
    tracing::info!("{:#?}", account_assets);

    Ok(())
}
