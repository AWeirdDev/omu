use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::boilerplate_flags_as_u8;

use super::{Attachment, ChannelType, HexCode, Role, User};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub author: User,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<User>,
    pub mention_roles: Vec<String>,
    pub mention_channels: Option<Vec<ChannelMention>>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,

    /// used for validating a message was sent
    pub nonce: Option<Nounce>,

    pub pinned: bool,
    pub webhook_id: Option<String>,

    #[serde(rename = "type")]
    pub type_: MessageType,
    // pub activity: Option<MessageActivity>,
    // pub application: Option<Application>,
    // pub message_reference: Option<MessageReference>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Nounce {
    Str(String),
    Integer(u64),
}

bitflags! {
    #[derive(Debug)]
    pub struct MessageType: u8 {
        const DEFAULT = 0;
        const RECIPIENT_ADD = 1;
        const RECIPIENT_REMOVE = 2;
        const CALL = 3;
        const CHANNEL_NAME_CHANGE = 4;
        const CHANNEL_ICON_CHANGE = 5;
        const CHANNEL_PINNED_MESSAGE = 6;
        const USER_JOIN = 7;
        const GUILD_BOOST = 8;
        const GUILD_BOOST_TIER_1 = 9;
        const GUILD_BOOST_TIER_2 = 10;
        const GUILD_BOOST_TIER_3 = 11;
        const CHANNEL_FOLLOW_ADD = 12;
        const GUILD_DISCOVERY_DISQUALIFIED = 14;
        const GUILD_DISCOVERY_REQUALIFIED = 15;
        const GUILD_DISCOVERY_GRACE_PERIOD_INITIAL_WARNING = 16;
        const GUILD_DISCOVERY_GRACE_PERIOD_FINAL_WARNING = 17;
        const THREAD_CREATED = 18;
        const REPLY = 19;
        const CHAT_INPUT_COMMAND = 20;
        const THREAD_STARTER_MESSAGE = 21;
        const GUILD_INVITE_REMINDER = 22;
        const CONTEXT_MENU_COMMAND = 23;
        const AUTO_MODERATION_ACTION = 24;
        const ROLE_SUBSCRIPTION_PURCHASE = 25;
        const INTERACTION_PREMIUM_UPSELL = 26;
        const STAGE_START = 27;
        const STAGE_END = 28;
        const STAGE_SPEAKER = 29;
        const STAGE_TOPIC = 31;
        const GUILD_APPLICATION_PREMIUM_SUBSCRIPTION = 32;
        const GUILD_INCIDENT_ALERT_MODE_ENABLED = 36;
        const GUILD_INCIDENT_ALERT_MODE_DISABLED = 37;
        const GUILD_INCIDENT_REPORT_RAID = 38;
        const GUILD_INCIDENT_REPORT_FALSE_ALARM = 39;
        const PURCHASE_NOTIFICATION = 44;
        const POLL_RESULT = 46;
    }
}
boilerplate_flags_as_u8!(MessageType);

#[derive(Debug, Serialize, Deserialize)]
pub struct Embed {
    pub title: Option<String>,

    #[serde(rename = "type")]
    pub type_: Option<EmbedType>,

    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<HexCode>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EmbedType {
    #[serde(rename = "rich")]
    Rich,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "gifv")]
    Gifv,
    #[serde(rename = "article")]
    Article,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "poll_result")]
    PollResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<usize>,
    pub width: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<usize>,
    pub width: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedVideo {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<usize>,
    pub width: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelMention {
    pub id: String,
    pub guild_id: String,

    #[serde(rename = "type")]
    pub type_: ChannelType,

    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    pub count: usize,
    pub count_details: ReactionDetails,
    pub me: bool,
    pub me_burst: bool,
    pub emoji: Emoji,

    /// HEX colors used for super reaction
    pub burst_colors: Vec<HexCode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionDetails {
    pub burst: usize,
    pub normal: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Emoji {
    pub id: Option<String>,
    pub name: Option<String>,
    pub roles: Option<Vec<Role>>,
    pub user: Option<User>,

    /// whether this emoji must be wrapped in colons
    pub requires_colons: Option<bool>,

    pub managed: Option<bool>,
    pub animated: Option<bool>,

    /// whether this emoji can be used, may be false due to loss of Server Boosts
    pub available: Option<bool>,
}
