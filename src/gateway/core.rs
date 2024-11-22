use std::{borrow::Cow, pin::Pin};

use anyhow::Result;
use futures_util::{stream::StreamExt, SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    tungstenite::protocol::{frame::coding::CloseCode, CloseFrame, Message},
    MaybeTlsStream, WebSocketStream,
};

use super::{get_sharding, GatewayEvent, Intents};

pub enum Status {
    Establishing,
    Established,
    Closed,
}

pub struct Gateway {
    pub stream: Option<Pin<Box<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    pub status: Status,
    pub heartbeat_interval: Option<usize>,
    pub sharding: Option<(u64, u64)>,
    pub last_sequence_number: Option<u64>,
}

impl Gateway {
    /// Connects to the gateway and returns a new [`Gateway`].
    pub async fn new_connection(endpoint: &str) -> Result<Self> {
        let (stream, _) = tokio_tungstenite::connect_async(endpoint).await?;

        Ok(Self {
            stream: Some(Box::pin(stream)),
            status: Status::Establishing,
            heartbeat_interval: None,
            sharding: None,
            last_sequence_number: None,
        })
    }

    /// Sets the sharding for the gateway.
    ///
    /// ```rust
    /// let gateway: Gateway = Gateway::new_connection("wss://gateway.discord.gg/?v=10&encoding=json")
    ///     .await?
    ///     .with_guild_sharding(123456789, 10);
    /// ```
    pub fn with_guild_sharding(mut self, guild_id: u64, total_shards: u64) -> Self {
        self.sharding = Some(get_sharding(guild_id, total_shards));
        self
    }

    /// Disconnects from the gateway.
    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(stream) = self.stream.take() {
            let mut ws_stream = Pin::into_inner(stream);
            ws_stream
                .close(Some(CloseFrame {
                    code: CloseCode::Normal,
                    reason: Cow::from("Disconnected"),
                }))
                .await?;

            Ok(())
        } else {
            Err(anyhow::anyhow!("Already disconnected"))
        }
    }

    /// Read one message at a time.
    pub async fn next(&mut self) -> Result<Option<Message>> {
        if let Some(stream) = self.stream.as_mut() {
            let (_, mut read) = stream.split();
            let message = read.next().await;
            if let Some(msg) = message {
                Ok(Some(msg?))
            } else {
                Ok(None)
            }
        } else {
            Err(anyhow::anyhow!("Already disconnected"))
        }
    }

    /// Send a message.
    pub async fn send(&mut self, message: Message) -> Result<()> {
        if let Some(stream) = self.stream.as_mut() {
            let (mut write, _) = stream.split();
            write.send(message).await?;

            Ok(())
        } else {
            Err(anyhow::anyhow!("Already disconnected"))
        }
    }

    /// Sends a heartbeat ACK. (op code: 11)
    pub async fn heartbeat(&mut self) -> Result<()> {
        self.send(
            super::event::GatewayEvent {
                op_code: 11,
                data: None,
                sequence: None,
            }
            .into(),
        )
        .await?;
        Ok(())
    }

    /// Authenticates with the gateway.
    /// ```rust
    /// use omu::Intents;
    ///
    /// // with intents
    /// gateway.authenticate("some token", Some(Intents::GUILD_MESSAGES | Intents::GUILD_MEMBERS)).await?;
    ///
    /// // without intents
    /// gateway.authenticate("some token", None).await?;
    /// ```
    pub async fn authenticate<K: ToString>(
        &mut self,
        token: K,
        intents: Option<Intents>,
    ) -> Result<()> {
        self.send(
            super::event::GatewayEvent::new_identify(
                token.to_string(),
                super::event::IdentifyConnectionProperty {
                    os: "linux".to_string(),
                    browser: "rust".to_string(),
                    device: "rust".to_string(),
                },
                Some(false),
                Some(50),
                self.sharding,
                None,
                intents.map(|i| i.into()),
            )
            .into(),
        )
        .await?;
        Ok(())
    }

    /// Authenticates with the gateway and returns the first event received, which is the result.
    pub async fn authenticate_flow<K: ToString>(
        &mut self,
        token: K,
        intents: Option<Intents>,
    ) -> Result<GatewayEvent> {
        self.authenticate(token, intents).await?;

        if let Some(message) = self.next().await? {
            let event: GatewayEvent = message.into();
            Ok(event)
        } else {
            Err(anyhow::anyhow!("Failed to authenticate (no response)"))
        }
    }
}
