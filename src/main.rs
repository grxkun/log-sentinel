use log_sentinel::DataPlatformClient;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let client = DataPlatformClient::new()?;

    client.initialize(true).await?;

    // Demonstrate configuration loading
    println!("Configuration loaded: {:?}", client.config());

    // Demonstrate creator features
    let creator = client.creator().get_profile("123").await?;
    println!("Creator profile: {:?}", creator);

    let balance = client.creator().get_balance("123").await?;
    println!("Creator balance: {:?}", balance);

    // Demonstrate verification features
    let verified = client.verification().check_verification("0x123").await?;
    println!("Address verified: {}", verified);

    let addresses = client.verification().get_addresses("user1").await?;
    println!("Verified addresses: {:?}", addresses);

    Ok(())
}