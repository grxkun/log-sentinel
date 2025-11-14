use async_trait::async_trait;
use crate::config::Config;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait CreatorAccess {
    async fn get_profile(&self, creator_id: &str) -> anyhow::Result<Creator>;
    async fn get_balance(&self, creator_id: &str) -> anyhow::Result<Balance>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Creator {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub creator_id: String,
    pub amount: f64,
}

pub struct CreatorClient {
    config: Config,
}

impl CreatorClient {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl CreatorAccess for CreatorClient {
    async fn get_profile(&self, creator_id: &str) -> anyhow::Result<Creator> {
        // Simulate API call
        println!("Fetching creator profile from {} for {}", self.config.api_base_url, creator_id);
        Ok(Creator {
            id: creator_id.to_string(),
            name: format!("Creator {}", creator_id),
        })
    }

    async fn get_balance(&self, creator_id: &str) -> anyhow::Result<Balance> {
        // Simulate API call
        println!("Fetching balance from {} for {}", self.config.api_base_url, creator_id);
        Ok(Balance {
            creator_id: creator_id.to_string(),
            amount: 100.0,
        })
    }
}