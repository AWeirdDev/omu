use anyhow::Result;
use omu::{Client, Intents};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::new(
        &dotenv::var("MY_TOKEN")?,
        Some(
            Intents::MESSAGE_CONTENT
                | Intents::GUILD_MESSAGES
                | Intents::DIRECT_MESSAGES
                | Intents::GUILDS,
        ),
    );
    client.run().await?;

    loop {
        match client.next().await {
            Ok(event) => println!("{:?}", event),
            Err(err) => println!("error: {}", err),
        }
    }
}
