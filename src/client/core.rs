use std::sync::Arc;

use anyhow::{anyhow, Result};
use tokio::sync::Mutex;

use crate::gateway::{Gateway, GatewayEvent, Intents, RawGatewayEvent, Rx};

pub struct Client {
    pub gateway: Arc<Mutex<Option<Gateway>>>,
    pub token: String,
    pub intents: Option<Intents>,
    pub rx: Option<Rx>,
}

impl Client {
    pub fn new<K: ToString>(token: K, intents: Option<Intents>) -> Self {
        Self {
            gateway: Arc::new(Mutex::new(None)),
            token: token.to_string(),
            intents,
            rx: None,
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
            .authenticate(&*self.token, self.intents.clone())
            .await?;

        if let Some(data) = gateway.next().await? {
            let event: RawGatewayEvent = data.into();
            match event.get_event_data()? {
                GatewayEvent::Hello(hello) => {
                    gateway.heartbeat_interval = Some(hello.heartbeat_interval);
                }
                _ => {
                    return Err(anyhow!(
                    "unrecognized data type after authentication (expected GatewayEvent::Hello)"
                ))
                }
            }
        } else {
            return Err(anyhow!(
                "no data received after authentication (expected GatewayEvent::Hello)"
            ));
        }

        self.gateway = Arc::new(Mutex::new(Some(gateway)));

        Ok(())
    }

    /// Iterates over the gateway and returns the next event data.
    /// Unlike `Gateway::next` (which returns a raw `Message`), this returns a `GatewayEvent`, a typed enum.
    pub async fn next(&mut self) -> Result<GatewayEvent> {
        if let Some(rx) = self.rx.as_mut() {
            if let Some(message) = rx.recv().await {
                let event: RawGatewayEvent = message.into();
                return event.get_event_data();
            }
        }

        Err(anyhow!("no data received"))
    }

    pub async fn run(&mut self) -> Result<()> {
        self.connect().await?;

        let mut gateway = self.gateway.lock().await;

        if let Some(gw) = gateway.as_mut() {
            let rx = gw.run().await?;
            self.rx = Some(rx);
        }

        Ok(())
    }
}
