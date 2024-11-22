omu is a discord api wrapper for rust.

let's just hope we can have a simple, non-complex api library.

```rust
use omu::{Client, Intents};

#[tokio::main]
async fn main() {
    let client = Client::new(
        "TOKEN",  // your token
        Some(Intents::MESSAGE_CONTENT | Intents::GUILD_PRESENCE)  // intents
    );

    // ... logics

    client.connect().await?;
}
```
