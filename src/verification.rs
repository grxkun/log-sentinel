use async_trait::async_trait;
use crate::config::Config;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait VerificationAccess {
    async fn check_verification(&self, address: &str) -> anyhow::Result<bool>;
    async fn get_addresses(&self, user_id: &str) -> anyhow::Result<Vec<VerifiedAddress>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifiedAddress {
    pub address: String,
    pub verified: bool,
}

pub struct VerificationClient {
    config: Config,
}

impl VerificationClient {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl VerificationAccess for VerificationClient {
    async fn check_verification(&self, address: &str) -> anyhow::Result<bool> {
        // Simulate API call
        println!("Checking verification for {} via {}", address, self.config.api_base_url);
        Ok(true)
    }

    async fn get_addresses(&self, user_id: &str) -> anyhow::Result<Vec<VerifiedAddress>> {
        // Simulate API call
        println!("Getting addresses for {} via {}", user_id, self.config.api_base_url);
        Ok(vec![VerifiedAddress {
            address: "0x123".to_string(),
            verified: true,
        }])
    }
}