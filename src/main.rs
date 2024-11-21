use anyhow::Result;

mod gateway;
use gateway::{Gateway, GatewayEvent, Intents};

#[tokio::main]
async fn main() -> Result<()> {
    let mut gateway =
        Gateway::connect("wss://gateway.discord.gg/?v=10&encoding=json".to_string()).await?;

    let message = gateway.next().await?;
    if let Some(message) = message {
        let event: GatewayEvent = message.into();
        println!("{:?}", event);
    }

    gateway.authenticate("hello", None).await?;
    let message = gateway.next().await?;
    if let Some(message) = message {
        let event: GatewayEvent = message.into();
        println!("{:?}", event);
    }
    
    gateway.disconnect().await?;

    Ok(())
}
