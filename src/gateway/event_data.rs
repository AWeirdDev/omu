use crate::dataclasses::{Message, PartialGuild, User};

use ijson::IValue;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum GatewayEvent {
    Ready(ReadyData),
    Hello(HelloData),
    MessageCreate(MessageCreateData),
    HeartbeatAcknowledgement,
    Heartbeat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadyData {
    #[serde(rename = "v")]
    pub version: u8,
    pub user: User,
    pub guilds: Vec<PartialGuild>,
    pub session_id: String,
    pub resume_gateway_url: String,
    pub shard: Option<(u64, u64)>,
    pub application: IValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloData {
    pub heartbeat_interval: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageCreateData {
    pub guild_id: Option<String>,
    pub message: Message,
    // pub member
    // pub mentions
}
