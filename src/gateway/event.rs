use crate::dataclasses;

use super::{
    event_data::{GatewayEvent, HelloData, ReadyData},
    Intents, MessageCreateData,
};

use anyhow::{anyhow, Result};
use ijson::{ijson, IValue};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct RawGatewayEvent {
    #[serde(rename = "op")]
    pub op_code: u32,
    #[serde(rename = "d")]
    pub data: Option<IValue>,
    #[serde(rename = "s")]
    pub sequence: Option<u64>,
    #[serde(rename = "t")]
    pub t: Option<String>,
}

impl From<Message> for RawGatewayEvent {
    fn from(value: Message) -> Self {
        let t = match value {
            Message::Text(event) => event,
            Message::Frame(frame) => frame.to_string(),
            _ => {
                panic!("unknown message! {}", value);
            }
        };

        serde_json::from_str::<Self>(t.as_str()).unwrap()
    }
}

impl From<RawGatewayEvent> for Message {
    fn from(val: RawGatewayEvent) -> Self {
        Message::Text(serde_json::to_string(&val).unwrap())
    }
}

impl RawGatewayEvent {
    /// Creates a new "Identify" structure. Used to trigger the initial handshake with the gateway.
    /// # Arguments
    /// * `token` - The token of the bot.
    /// * `properties` - The connection properties of the bot.
    /// * `compress` - Whether this connection supports compression of packets.
    /// * `large_threshold` - Value between 50 and 250, total number of members where the gateway will stop sending offline members in the guild member list.
    /// * `shard` - The shard of the bot. An tuple of two integers (`shard_id`, `num_shards`)
    /// * `presence` - The presence of the bot.
    /// * `intents` - Gateway intents to receive.
    pub fn new_identify(
        token: &str,
        properties: IdentifyConnectionProperty,
        compress: Option<bool>,
        large_threshold: Option<u8>,
        shard: Option<(u64, u64)>,
        presence: Option<ijson::IValue>,
        intents: Option<u64>,
    ) -> Self {
        let mut data = ijson!({
            "token": token,
            "properties": properties,
            "compress": compress,
            "large_threshold": large_threshold,
            "presence": presence,
            "intents": intents.unwrap_or(Intents::empty().into()),
        });

        if let Some((id, total)) = shard {
            data["shard"] = ijson!([id, total]);
        }

        Self {
            op_code: 2,
            data: Some(data),
            sequence: None,
            t: None,
        }
    }

    pub fn get_event_data(&self) -> Result<GatewayEvent> {
        if let Some(data) = &self.data {
            let e = match self.op_code {
                0 => match self.t.as_deref().unwrap() {
                    "READY" => GatewayEvent::Ready(ijson::from_value::<ReadyData>(data)?),
                    "MESSAGE_CREATE" => GatewayEvent::MessageCreate(MessageCreateData {
                        guild_id: data
                            .get("guild_id")
                            .map(|v| v.as_string().unwrap().to_string()),
                        message: ijson::from_value::<dataclasses::Message>(data)?,
                    }),
                    _ => return Err(anyhow!("unrecognized data type. raw: {:?}", self)),
                },

                10 => GatewayEvent::Hello(HelloData {
                    heartbeat_interval: data["heartbeat_interval"]
                        .as_number()
                        .unwrap()
                        .to_u64()
                        .unwrap(),
                }),
                _ => return Err(anyhow!("unknown op code and data:\n{:?}", self)),
            };
            Ok(e)
        } else {
            Ok(match self.op_code {
                1 => GatewayEvent::Heartbeat,
                11 => GatewayEvent::HeartbeatAcknowledgement,
                _ => return Err(anyhow!("unrecognized data type. raw: {:?}", self)),
            })
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifyConnectionProperty {
    /// The operating system.
    pub os: String,
    /// The library name.
    pub browser: String,
    /// The library name.
    pub device: String,
}
