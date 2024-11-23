use std::pin::Pin;

use anyhow::{anyhow, Result};

use crate::{
    gateway::{Gateway, Intents},
    GatewayEvent, GatewayEventData,
};

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

    /// Connects to the gateway. This only registers a gateway object inside the client struct.
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

    /// Iterates over the gateway and returns the next event data.
    /// Unlike `Gateway::next` (which returns a raw `Message`), this returns a `GatewayEventData`, a typed enum.
    ///
    /// # Example
    /// ```rust
    /// let client = Client::new("TOKEN", None);
    /// client.connect().await?;
    ///
    /// while let Ok(data) = client.next().await {
    ///     match data {
    ///         GatewayEventData::Ready(ready) => {
    ///             println!("Ready: {:#?}", ready);
    ///         }
    ///         _ => {}
    ///     }
    /// }
    /// ```
    pub async fn next(&mut self) -> Result<GatewayEventData> {
        if let Some(gateway) = self.gateway.as_mut() {
            if let Some(message) = gateway.next().await? {
                let event: GatewayEvent = message.into();
                let data = event.get_event_data()?;

                return Ok(data);
            }
        }

        Err(anyhow!("no data received"))
    }
}
