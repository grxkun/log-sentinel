use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_base_url: String,
    pub api_key: String,
    pub database_url: String,
    pub backfill_start_point: u64,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        Ok(Self {
            api_base_url: std::env::var("API_BASE_URL")?,
            api_key: std::env::var("API_KEY")?,
            database_url: std::env::var("DATABASE_URL")?,
            backfill_start_point: std::env::var("BACKFILL_START_POINT")?.parse()?,
        })
    }
}