use super::{
    event_data::{GatewayEventData, HelloData, ReadyData},
    Intents, Message,
};

use anyhow::{anyhow, Result};
use ijson::{ijson, IValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayEvent {
    #[serde(rename = "op")]
    pub op_code: u32,
    #[serde(rename = "d")]
    pub data: Option<IValue>,
    #[serde(rename = "s")]
    pub sequence: Option<u64>,
}

impl From<Message> for GatewayEvent {
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

impl Into<Message> for GatewayEvent {
    fn into(self) -> Message {
        Message::Text(serde_json::to_string(&self).unwrap())
    }
}

impl GatewayEvent {
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
        token: String,
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
        }
    }

    /// Gets the event data in the `data` field but in a typed enum.
    ///
    /// # Example
    /// ```rust
    /// let event: GatewayEvent = GatewayEvent { ... };
    /// let data: GatewayEventData = event.get_event_data()?;
    /// ```
    pub fn get_event_data(&self) -> Result<GatewayEventData> {
        if let Some(data) = &self.data {
            let e = match self.op_code {
                0 => GatewayEventData::Ready(ijson::from_value::<ReadyData>(data).unwrap()),
                10 => GatewayEventData::Hello(HelloData {
                    heartbeat_interval: data["heartbeat_interval"]
                        .as_number()
                        .unwrap()
                        .to_usize()
                        .unwrap(),
                }),
                _ => panic!("unknown op code! {}", self.op_code),
            };
            Ok(e)
        } else {
            Err(anyhow!("unrecognized data type. raw: {:?}", self.data))
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
