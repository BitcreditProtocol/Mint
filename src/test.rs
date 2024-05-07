#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use mokshamint::config::{DatabaseConfig, LightningFeeConfig};
    use mokshamint::lightning::lnd::LndLightningSettings;
    use mokshamint::lightning::LightningType;
    use mokshamint::mint::MintBuilder;
    use mokshamint::server::run_server;

    #[tokio::test]
    async fn run_lnd_local_mint() -> anyhow::Result<()> {
        dotenv().ok();

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
        };

        let mint = MintBuilder::new()
            .with_db(database_config)
            .with_fee(Some(lighting_fee_config))
            .with_lightning(ln_type)
            .with_private_key("my_private_key".to_string())
            .build()
            .await;

        run_server(mint?).await
    }
}
