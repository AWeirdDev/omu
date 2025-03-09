use anyhow::Result;
use omu::{
    dataclasses::{AllowedMention, TextChannel},
    Client, GatewayEvent, Intents,
};

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
                println!("{event:?}");
                if let GatewayEvent::MessageCreate(mc) = event {
                    if &mc.message.content == "hello" {
                        let channel: TextChannel = mc.message.fetch_channel().await?.into();

                        channel
                            .prepare_send()
                            .content("Wow, that's miserable. @everyone".to_string())
                            .allowed_mentions(AllowedMention::builder().build())
                            .send()
                            .await?;
                    }
                }
            }
            Err(err) => println!("error: {}", err),
        }
    }
}
