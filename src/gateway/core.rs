use std::{borrow::Cow, pin::Pin};

use anyhow::Result;
use futures_util::{stream::StreamExt, SinkExt};
use ijson::ijson;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    tungstenite::protocol::{frame::coding::CloseCode, CloseFrame, Message},
    MaybeTlsStream, WebSocketStream,
};

use super::Intents;

pub enum Status {
    Establishing,
    Established,
    Closed,
}

pub struct Gateway {
    pub stream: Option<Pin<Box<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    pub status: Status,
    pub heartbeat_interval: Option<usize>,
}

impl Gateway {
    /// Connects to the gateway.
    pub async fn connect(endpoint: String) -> Result<Self> {
        let (stream, _) = tokio_tungstenite::connect_async(endpoint).await?;

        Ok(Self {
            stream: Some(Box::pin(stream)),
            status: Status::Establishing,
            heartbeat_interval: None,
        })
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
    pub async fn authenticate(&mut self, token: &str, intents: Option<Intents>) -> Result<()> {
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
                None,
                None,
                intents.map(|i| i.into()),
            )
            .into(),
        )
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() -> Result<()> {
        let mut gateway =
            Gateway::connect("wss://gateway.discord.gg/?v=10&encoding=json".to_string()).await?;
        gateway.disconnect().await?;

        Ok(())
    }
}
