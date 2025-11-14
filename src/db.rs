use async_trait::async_trait;

#[async_trait]
pub trait DataStore {
    async fn connect(&self) -> anyhow::Result<()>;
    async fn start_historical_backfill(&self) -> anyhow::Result<()>;
}

pub struct PostgresClient {
    config: crate::config::Config,
}

impl PostgresClient {
    pub fn new(config: crate::config::Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl DataStore for PostgresClient {
    async fn connect(&self) -> anyhow::Result<()> {
        // Simulate database connection
        println!("Connecting to database: {}", self.config.database_url);
        Ok(())
    }

    async fn start_historical_backfill(&self) -> anyhow::Result<()> {
        println!("Starting historical backfill from {}", self.config.backfill_start_point);
        Ok(())
    }
}