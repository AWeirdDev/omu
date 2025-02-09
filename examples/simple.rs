use anyhow::Result;
use omu::{Client, Intents};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::new(
        &dotenv::var("MY_TOKEN")?,
        Some(Intents::MESSAGE_CONTENT | Intents::GUILD_MESSAGES),
    );
    client.run().await?;

    loop {
        if let Ok(data) = client.next().await {
            println!("{data:?}");
        }
    }
}
