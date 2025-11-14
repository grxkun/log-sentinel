pub mod config;
pub mod db;
pub mod creator;
pub mod verification;

use std::sync::Arc;

pub struct DataPlatformClient {
    config: config::Config,
    db: Arc<dyn db::DataStore + Send + Sync>,
    creator: Arc<dyn creator::CreatorAccess + Send + Sync>,
    verification: Arc<dyn verification::VerificationAccess + Send + Sync>,
}

impl DataPlatformClient {
    pub fn new() -> anyhow::Result<Self> {
        let config = config::Config::load()?;
        let db = Arc::new(db::PostgresClient::new(config.clone()));
        let creator = Arc::new(creator::CreatorClient::new(config.clone()));
        let verification = Arc::new(verification::VerificationClient::new(config.clone()));
        Ok(Self {
            config,
            db,
            creator,
            verification,
        })
    }

    pub async fn initialize(&self, run_backfill: bool) -> anyhow::Result<()> {
        self.db.connect().await?;
        if run_backfill {
            self.db.start_historical_backfill().await?;
        }
        Ok(())
    }

    pub fn config(&self) -> &config::Config {
        &self.config
    }

    pub fn db_access(&self) -> &Arc<dyn db::DataStore + Send + Sync> {
        &self.db
    }

    pub fn creator(&self) -> &Arc<dyn creator::CreatorAccess + Send + Sync> {
        &self.creator
    }

    pub fn verification(&self) -> &Arc<dyn verification::VerificationAccess + Send + Sync> {
        &self.verification
    }
}