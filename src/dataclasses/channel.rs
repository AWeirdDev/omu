use std::{marker::PhantomData, sync::Arc};

use anyhow::Result;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    boilerplate_flags,
    http::{client::HttpClient, http_messages::PrepareCreateMessageBuilder},
};

use super::{HttpAttachable, Snowflake, User};

/// To convert this directly into the typed version of a channel, use [`Channel::into`].
#[derive(Debug, Serialize, Deserialize)]
pub struct Channel<T> {
    #[serde(skip)]
    _marker: PhantomData<T>,

    #[serde(skip)]
    http: Option<Arc<HttpClient>>,

    pub id: Snowflake,
    #[serde(rename = "type")]
    pub type_: ChannelType,
    pub guild_id: Option<Snowflake>,
    pub position: Option<u16>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: Option<String>,

    /// The channel topic.
    /// (0-4096 characters for GUILD_FORUM and GUILD_MEDIA channels, 0-1024 characters for all others)
    pub topic: Option<String>,

    pub nsfw: Option<bool>,
    pub last_message_id: Option<Snowflake>,

    pub bitrate: Option<usize>,
    pub user_limit: Option<usize>,
    pub rate_limit_per_user: Option<usize>,

    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<Snowflake>,
    pub application_id: Option<Snowflake>,

    pub managed: Option<bool>,

    /// for guild channels: id of the parent category for a channel (each parent category can contain up to 50 channels);
    /// for threads: id of the text channel this thread was created
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,

    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<VideoQualityMode>,

    pub message_count: Option<usize>,

    /// an approximate count of users in a thread, stops counting at 50
    pub member_count: Option<u8>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub default_auto_archive_duration: Option<AutoArchiveDuration>,

    pub permissions: Option<String>,
    pub flags: Option<ChannelFlags>,

    /// number of messages ever sent in a thread, it's similar to `message_count` on message creation,
    /// but will not decrement the number when a message is deleted
    pub total_message_sent: Option<usize>,

    pub available_tags: Option<Vec<ForumTag>>,
    pub applied_tags: Option<Vec<String>>,
    pub default_reaction_emoji: Option<DefaultForumReactionEmoji>,
    pub default_thread_rate_limit_per_user: Option<usize>,

    /// Defaults to null, which indicates a preferred sort order hasn't been set by a channel admin
    pub default_sort_order: Option<ForumSortOrder>,

    /// Defaults to 0, which indicates a layout view has not been set by a channel admin
    pub default_forum_layout: Option<ForumLayout>,
}

impl<T> HttpAttachable for Channel<T> {
    fn attach(&mut self, http: Arc<HttpClient>) {
        self.http = Some(http);
    }
}

#[derive(Debug)]
pub struct TextChannel {
    http: Option<Arc<HttpClient>>,

    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub position: u16,
    pub permission_overwrites: Vec<Overwrite>,
    pub name: String,
    pub topic: Option<String>,
    pub nsfw: bool,
    pub last_message_id: Option<Snowflake>,
    pub rate_limit_per_user: Option<usize>,
    pub last_pin_timestamp: Option<String>,
    pub permissions: Option<String>,
    pub flags: Option<ChannelFlags>,
}

impl<'a> Channel<TextChannel> {
    fn _to(self) -> TextChannel {
        TextChannel {
            http: self.http,
            id: self.id,
            guild_id: self.guild_id.unwrap(),
            position: self.position.unwrap(),
            permission_overwrites: self.permission_overwrites.unwrap(),
            name: self.name.unwrap(),
            topic: self.topic,
            nsfw: self.nsfw.unwrap(),
            last_message_id: self.last_message_id,
            rate_limit_per_user: self.rate_limit_per_user,
            last_pin_timestamp: self.last_pin_timestamp,
            permissions: self.permissions,
            flags: self.flags,
        }
    }

    pub fn prepare_send(&'a self) -> PrepareCreateMessageBuilder<'a> {
        PrepareCreateMessageBuilder::new(&self.http.as_ref().unwrap(), &self.id)
    }
}

impl From<Channel<TextChannel>> for TextChannel {
    /// Converts directly into a typed text channel.
    fn from(value: Channel<TextChannel>) -> Self {
        value._to()
    }
}

impl<'a> TextChannel {
    pub fn prepare_send(&'a self) -> PrepareCreateMessageBuilder<'a> {
        PrepareCreateMessageBuilder::new(&self.http.as_ref().unwrap(), &self.id)
    }
}

#[derive(Debug)]
pub struct VoiceChannel {
    http: Option<Arc<HttpClient>>,

    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub position: u16,
    pub permission_overwrites: Vec<Overwrite>,
    pub name: String,
    pub bitrate: Option<usize>,
    pub user_limit: Option<usize>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<VideoQualityMode>,
    pub permissions: Option<String>,
    pub flags: Option<ChannelFlags>,
}

impl<'a> Channel<VoiceChannel> {
    fn _to(self) -> VoiceChannel {
        VoiceChannel {
            http: self.http,
            id: self.id,
            guild_id: self.guild_id.unwrap(),
            position: self.position.unwrap(),
            permission_overwrites: self.permission_overwrites.unwrap(),
            name: self.name.unwrap(),
            bitrate: self.bitrate,
            user_limit: self.user_limit,
            rtc_region: self.rtc_region,
            video_quality_mode: self.video_quality_mode,
            permissions: self.permissions,
            flags: self.flags,
        }
    }

    pub fn prepare_send(&'a self) -> PrepareCreateMessageBuilder<'a> {
        PrepareCreateMessageBuilder::new(&self.http.as_ref().unwrap(), &self.id)
    }
}

impl From<Channel<VoiceChannel>> for VoiceChannel {
    /// Converts directly into a typed voice channel.
    fn from(value: Channel<VoiceChannel>) -> Self {
        value._to()
    }
}

impl<'a> VoiceChannel {
    pub fn prepare_send(&'a self) -> PrepareCreateMessageBuilder<'a> {
        PrepareCreateMessageBuilder::new(&self.http.as_ref().unwrap(), &self.id)
    }
}

#[derive(Debug)]
pub struct Thread;

bitflags! {
    #[derive(Debug)]
    pub struct ChannelType: u64 {
        const GUILD_TEXT      = 1 << 0;
        const DM             = 1 << 1;
        const GUILD_VOICE    = 1 << 2;
        const GROUP_DM       = 1 << 3;
        const GUILD_CATEGORY = 1 << 4;
        const GUILD_ANNOUNCEMENT = 1 << 5;
        const ANNOUNCEMENT_THREAD = 1 << 10;
        const PUBLIC_THREAD     = 1 << 11;
        const PRIVATE_THREAD    = 1 << 12;
        const GUILD_STAGE_VOICE = 1 << 13;
        const GUILD_DIRECTORY  = 1 << 14;
        const GUILD_FORUM      = 1 << 15;
        const GUILD_MEDIA      = 1 << 16;
    }
}

boilerplate_flags!(ChannelType);

bitflags! {
    #[derive(Debug)]
    pub struct ChannelFlags: u64 {
        const PINNED = 1 << 1;
        const REQUIRED_TAG = 1 << 4;
        const HIDE_MEDIA_DOWNLOAD_OPTIONS = 1 << 15;
    }
}
boilerplate_flags!(ChannelFlags);

#[derive(Debug, Serialize, Deserialize)]
pub struct Overwrite {
    /// Role or user ID.
    pub id: Snowflake,

    #[serde(rename = "type")]
    pub type_: OverwriteType,

    pub allow: Permissions,
    pub deny: Permissions,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum OverwriteType {
    Role = 0,
    Member = 1,
}

bitflags! {
    #[derive(Debug)]
    pub struct Permissions: u64 {
        const CREATE_INSTANT_INVITE = 1 << 0;
        const KICK_MEMBERS = 1 << 1;
        const BAN_MEMBERS = 1 << 2;
        const ADMINISTRATOR = 1 << 3;
        const MANAGE_CHANNELS = 1 << 4;
        const MANAGE_GUILD = 1 << 5;
        const ADD_REACTIONS = 1 << 6;
        const VIEW_AUDIT_LOG = 1 << 7;
        const PRIORITY_SPEAKER = 1 << 8;
        const STREAM = 1 << 9;
        const VIEW_CHANNEL = 1 << 10;
        const SEND_MESSAGES = 1 << 11;
        const SEND_TTS_MESSAGES = 1 << 12;
        const MANAGE_MESSAGES = 1 << 13;
        const EMBED_LINKS = 1 << 14;
        const ATTACH_FILES = 1 << 15;
        const READ_MESSAGE_HISTORY = 1 << 16;
        const MENTION_EVERYONE = 1 << 17;
        const USE_EXTERNAL_EMOJIS = 1 << 18;
        const VIEW_GUILD_INSIGHTS = 1 << 19;
        const CONNECT = 1 << 20;
        const SPEAK = 1 << 21;
        const MUTE_MEMBERS = 1 << 22;
        const DEAFEN_MEMBERS = 1 << 23;
        const MOVE_MEMBERS = 1 << 24;
        const USE_VAD = 1 << 25;
        const CHANGE_NICKNAME = 1 << 26;
        const MANAGE_NICKNAMES = 1 << 27;
        const MANAGE_ROLES = 1 << 28;
        const MANAGE_WEBHOOKS = 1 << 29;
        const MANAGE_GUILD_EXPRESSIONS = 1 << 30;
        const USE_APPLICATION_COMMANDS = 1 << 31;
        const REQUEST_TO_SPEAK = 1 << 32;
        const MANAGE_EVENTS = 1 << 33;
        const MANAGE_THREADS = 1 << 34;
        const CREATE_PUBLIC_THREADS = 1 << 35;
        const CREATE_PRIVATE_THREADS = 1 << 36;
        const USE_EXTERNAL_STICKERS = 1 << 37;
        const SEND_MESSAGES_IN_THREADS = 1 << 38;
        const USE_EMBEDDED_ACTIVITIES = 1 << 39;
        const MODERATE_MEMBERS = 1 << 40;
        const VIEW_CREATOR_MONETIZATION_ANALYTICS = 1 << 41;
        const USE_SOUNDBOARD = 1 << 42;
        const CREATE_GUILD_EXPRESSIONS = 1 << 43;
        const CREATE_EVENTS = 1 << 44;
        const USE_EXTERNAL_SOUNDS = 1 << 45;
        const SEND_VOICE_MESSAGES = 1 << 46;
        const SEND_POLLS = 1 << 49;
        const USE_EXTERNAL_APPS = 1 << 50;
    }
}

impl Permissions {
    pub const ALL: Permissions = Permissions::all();
    pub const NONE: Permissions = Permissions::empty();
}

impl Serialize for Permissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.bits().to_string())
    }
}

impl<'de> Deserialize<'de> for Permissions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Permissions::from_bits_truncate(s.parse().unwrap()))
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum VideoQualityMode {
    Auto = 1,
    /// 720p
    Full = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u64,
    pub archive_timestamp: String,
    pub locked: bool,
    pub invitable: Option<bool>,
    pub create_timestamp: Option<String>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum AutoArchiveDuration {
    OneHour = 60,
    OneDay = 1440,
    ThreeDays = 4320,
    OneWeek = 10080,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForumTag {
    id: Snowflake,
    name: String,
    moderated: bool,
    emoji_id: Option<Snowflake>,
    emoji_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultForumReactionEmoji {
    emoji_id: Option<Snowflake>,
    emoji_name: Option<String>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ForumSortOrder {
    LatestActivity = 0,
    CreationDate = 1,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ForumLayout {
    NotSet = 0,
    ListView = 1,
    GalleryView = 2,
}
