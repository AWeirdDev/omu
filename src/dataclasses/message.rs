use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::boilerplate_flags;

use super::{Attachment, Channel, ChannelType, HexCode, Role, Thread, User};

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
    pub mention_channels: Option<Vec<mentions::ChannelMention>>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<embed::Embed>,
    pub reactions: Option<Vec<Reaction>>,

    /// used for validating a message was sent
    pub nonce: Option<Nounce>,

    pub pinned: bool,
    pub webhook_id: Option<String>,

    #[serde(rename = "type")]
    pub type_: MessageType,

    pub activity: Option<MessageActivity>,
    // pub application: Option<Application>,
    // pub message_reference: Option<MessageReference>,
    pub flags: Option<MessageFlags>,
    pub message_reference: Option<MessageReference>,
    pub message_snapshots: Option<Vec<MessageSnapshot>>,
    pub referenced_message: Option<MessageReference>,
    // interaction_metadata
    // interaction
    pub thread: Option<Channel<Thread>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageSnapshot {
    /// A partial message object. Contains a minimal subset of fields in the forwarded message.
    /// The current subset of message fields consists of:
    /// `type`, `content`, `embeds`, `attachments`, `timestamp`, `edited_timestamp`,
    /// `flags`, `mentions`, `mention_roles`, `stickers`, `sticker_items`, and `components`.
    ///
    /// While message snapshots are able to support nested snapshots, we (Discord) currently limit the depth of nesting to 1.
    message: Message,
}

bitflags! {
    #[derive(Debug)]
    pub struct MessageFlags: u64 {
        const CROSSPOSTED = 1 << 0;
        const IS_CROSSPOST = 1 << 1;
        const SUPPRESS_EMBEDS = 1 << 2;
        const SOURCE_MESSAGE_DELETED = 1 << 3;
        const URGENT = 1 << 4;
        const HAS_THREAD = 1 << 5;
        const EPHEMERAL = 1 << 6;
        const LOADING = 1 << 7;
        const FAILED_TO_MENTION_SOME_ROLES_IN_THREAD = 1 << 8;
        const SUPPRESS_NOTIFICATIONS = 1 << 12;
        const IS_VOICE_MESSAGE = 1 << 13;
        const HAS_SNAPSHOT = 1 << 14;
    }
}

boilerplate_flags!(MessageFlags);

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Nounce {
    Str(String),
    Integer(u64),
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    UserJoin = 7,
    GuildBoost = 8,
    GuildBoostTier1 = 9,
    GuildBoostTier2 = 10,
    GuildBoostTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    Reply = 19,
    ChatInputCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
    ContextMenuCommand = 23,
    AutoModerationAction = 24,
    RoleSubscriptionPurchase = 25,
    InteractionPremiumUpsell = 26,
    StageStart = 27,
    StageEnd = 28,
    StageSpeaker = 29,
    StageTopic = 31,
    GuildApplicationPremiumSubscription = 32,
    GuildIncidentAlertModeEnabled = 36,
    GuildIncidentAlertModeDisabled = 37,
    GuildIncidentReportRaid = 38,
    GuildIncidentReportFalseAlarm = 39,
    PurchaseNotification = 44,
    PollResult = 46,
}

pub mod embed {
    use super::*;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageActivity {
    type_: MessageActivityType,
    party_id: Option<String>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllowedMention {
    pub parse: Vec<AllowedMentionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AllowedMentionType {
    #[serde(rename = "roles")]
    Roles,
    #[serde(rename = "users")]
    Users,
    #[serde(rename = "everyone")]
    Everyone,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageReference {
    #[serde(rename = "type")]
    type_: MessageReferenceType,
    message_id: Option<String>,

    /// Required for forwards.
    channel_id: Option<String>,

    guild_id: Option<String>,
    fail_if_not_exists: Option<bool>,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum MessageReferenceType {
    /// Coupled Message Field: `reference_message`
    Default = 0,
    /// Coupled Message Field: `message_snapshot`
    Forward = 1,
}

pub mod mentions {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ChannelMention {
        pub id: String,
        pub guild_id: String,

        #[serde(rename = "type")]
        pub type_: ChannelType,

        pub name: String,
    }
}
