omu is a discord api wrapper for rust.

let's just hope we can have a simple, non-complex api library.

```rust
// unimplemented: SlashContext
use anyhow::Result;
use omu::{Client, Intents, SlashContext};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new(
        "TOKEN",  // your token
        Some(Intents::MESSAGE_CONTENT | Intents::GUILD_PRESENCE)  // intents
    )
        .slash(ping)
        .event(ready);

    client.run().await?;
    Ok(())
}

#[slash]
async fn ping(ctx: SlashContext) -> Result<()> {
    ctx.respond(":ping_pong: Pong!").await?;
    Ok(())
}

#[event(ready)]
async fn ready() {
    println!("Ready!");
}
```
