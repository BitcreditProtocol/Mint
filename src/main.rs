mod test;

use std::env;

use dotenv::dotenv;
use mokshamint::config::{DatabaseConfig, LightningFeeConfig};
use mokshamint::lightning::lnd::LndLightningSettings;
use mokshamint::lightning::LightningType;
use mokshamint::mint::MintBuilder;
use mokshamint::server::run_server;

#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    dotenv().ok();

    init_mint().await.unwrap();
}

async fn init_mint() -> anyhow::Result<()> {
    let lnd_settings = envy::prefixed("LND_")
        .from_env::<LndLightningSettings>()
        .expect("Please provide lnd info");

    let ln_type = LightningType::Lnd(lnd_settings);

    let lighting_fee_config = LightningFeeConfig {
        fee_percent: 0f32,
        fee_reserve_min: 4000,
    };

    let database_config = DatabaseConfig {
        db_url: "postgres://postgres:postgres@localhost:5432/moksha-mint".to_string(),
        max_connections: 1,
    };

    let mint = MintBuilder::new()
        .with_db(Option::from(database_config))
        .with_fee(Some(lighting_fee_config))
        .with_lightning(ln_type)
        .with_private_key("my_private_key".to_string())
        .build()
        .await
        .expect("Can't create mint");

    run_server(mint).await
}
