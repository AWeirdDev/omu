use anyhow::Result;
use omu::{Client, Intents};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::new(
        &dotenv::var("MY_TOKEN")?,
        Some(Intents::MESSAGE_CONTENT | Intents::GUILDS | Intents::DIRECT_MESSAGES),
    );
    client.run().await?;

    loop {
        match client.next().await {
            Ok(event) => println!("{:?}", event),
            Err(err) => println!("error: {}", err),
        }
    }
}
