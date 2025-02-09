omu is a simple rust discord api wrapper. it's as simple as it should be.

```rust
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
```

you handle events just like going through an iterator, and we do the rest, so that there's no structs, impls, or other weird stuff involved. cmon, no one likes them in your discord bot project.
