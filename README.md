omu is a simple rust discord api wrapper. it's as simple as it should be.

below is the redesigned api.

```rust
use anyhow::Result;
use omu::*;

// Make it constant to access from anywhere
const CLIENT: Client = Client::new(
    Some(Intents::MESSAGE_CONTENT
                | Intents::GUILD_MESSAGES
                | Intents::DIRECT_MESSAGES
                | Intents::GUILDS)
);

#[event(on_message)]
async fn on_message(message: Message) -> Result<()> {
  message.reply(format!("Hello, {}", message.author.mention())).await?;

  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    CLIENT.add(on_message);
    
    CLIENT.run(&dotenv::var("MY_TOKEN")).await?;
}
```

you handle events just like going through an iterator, and we do the rest, so that there's no structs, impls, or other weird stuff involved. cmon, no one likes them in your discord bot project.
