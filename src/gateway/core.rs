use std::{borrow::Cow, sync::Arc, time::Duration};

use anyhow::Result;
use futures_util::{stream::StreamExt, SinkExt};
use tokio::{
    net::TcpStream,
    sync::{
        mpsc::{self, UnboundedReceiver, UnboundedSender},
        Mutex,
    },
    time::interval,
};
use tokio_tungstenite::{
    tungstenite::protocol::{frame::coding::CloseCode, CloseFrame, Message},
    MaybeTlsStream, WebSocketStream,
};

use super::{get_sharding, Intents};

pub type Tx = UnboundedSender<Message>;
pub type Rx = UnboundedReceiver<Message>;

pub enum Status {
    Establishing,
    Established,
    Closed,
}

pub struct Gateway {
    pub stream: Arc<Mutex<Option<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    pub status: Status,
    pub heartbeat_interval: Option<u64>,
    pub sharding: Option<(u64, u64)>,
    pub last_sequence_number: Option<u64>,
}

impl Gateway {
    /// Connects to the gateway and returns a new [`Gateway`].
    pub async fn new_connection(endpoint: &str) -> Result<Self> {
        let (stream, _) = tokio_tungstenite::connect_async(endpoint).await?;

        Ok(Self {
            stream: Arc::new(Mutex::new(Some(stream))),
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
        let mut stream = self.stream.lock().await;
        if let Some(mut stream) = stream.take() {
            stream
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
        let mut stream = self.stream.lock().await;
        if let Some(stream) = stream.as_mut() {
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
        let mut stream = self.stream.lock().await;
        if let Some(stream) = stream.as_mut() {
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
            super::event::RawGatewayEvent {
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
    pub async fn authenticate(&mut self, token: &str, intents: Option<Intents>) -> Result<()> {
        self.send(
            super::event::RawGatewayEvent::new_identify(
                token,
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

    pub async fn run(&mut self) -> Result<Rx> {
        let interval_ms = self.heartbeat_interval.unwrap_or(5000);

        let (tx, rx) = mpsc::unbounded_channel::<Message>();

        tokio::spawn(Self::heartbeat_task(self.stream.clone(), interval_ms));
        tokio::spawn(Self::receive_task(self.stream.clone(), tx));

        Ok(rx)
    }

    async fn heartbeat_task(
        stream: Arc<Mutex<Option<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
        interval_ms: u64,
    ) {
        let mut interval = interval(Duration::from_millis(interval_ms));

        loop {
            println!("Sending heartbeat...");
            let message = Message::Text(r#"{"op":1,"d":null}"#.to_string());

            let mut stream = stream.lock().await;
            if let Some(stream) = stream.as_mut() {
                stream.send(message).await.ok();
            }

            interval.tick().await;
        }
    }

    async fn receive_task(
        stream: Arc<Mutex<Option<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
        tx: Tx,
    ) {
        loop {
            let mut lock = stream.lock().await;
            if let Some(ws_stream) = lock.as_mut() {
                if let Some(Ok(msg)) = ws_stream.next().await {
                    tx.send(msg).ok();
                }
            }
        }
    }
}
