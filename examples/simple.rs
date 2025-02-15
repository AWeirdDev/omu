use anyhow::Result;
use omu::{dataclasses::TextChannel, Client, GatewayEvent, Intents};

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
            Ok(event) => {
                if let GatewayEvent::MessageCreate(mc) = event {
                    if &mc.message.content == "hello" {
                        let channel = client
                            .http
                            .get_channel::<TextChannel>(&mc.message.channel_id)
                            .await?
                            .attach(client.http.clone());

                        channel
                            .message()
                            .content("Hello, World!".to_string())
                            .send()
                            .await?;
                    }
                }
            }
            Err(err) => println!("error: {}", err),
        }
    }
}
