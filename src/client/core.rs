use std::pin::Pin;

use anyhow::Result;

use crate::gateway::{Gateway, Intents};

pub struct Client {
    pub gateway: Option<Pin<Box<Gateway>>>,
    pub token: String,
    pub intents: Option<Intents>,
}

impl Client {
    pub fn new<K: ToString>(token: K, intents: Option<Intents>) -> Self {
        Self {
            gateway: None,
            token: token.to_string(),
            intents: intents,
        }
    }

    /// Connects to the gateway.
    /// 
    /// # Example
    /// ```rust
    /// use omu::Intents;
    ///
    /// let mut client = Client::new("token".to_string(), Some(Intents::GUILD_MESSAGES));
    /// client.connect().await?;
    /// ```
    pub async fn connect(&mut self) -> Result<()> {
        let mut gateway =
            Gateway::new_connection("wss://gateway.discord.gg/?v=10&encoding=json").await?;
        gateway
            .authenticate(self.token.clone(), self.intents.clone())
            .await?;

        self.gateway = Some(Box::pin(gateway));

        Ok(())
    }
}
