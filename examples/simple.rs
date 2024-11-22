use anyhow::Result;
use dotenv;

use omu::{Gateway, GatewayEvent};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let mut gateway =
        Gateway::new_connection("wss://gateway.discord.gg/?v=10&encoding=json").await?;

    let message = gateway.next().await?;
    if let Some(message) = message {
        let event: GatewayEvent = message.into();
        println!("{:?}", event);
    }

    let result = gateway.authenticate_flow(dotenv::var("TOKEN")?, None).await?;
    println!("{result:?}");

    gateway.disconnect().await?;

    Ok(())
}
