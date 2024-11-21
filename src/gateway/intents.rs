use bitflags::bitflags;

bitflags! {
    pub struct Intents: u64 {
        const GUILDS = 1 << 0;
        const GUILD_MEMBERS = 1 << 1;
        const GUILD_BANS = 1 << 2;
        const GUILD_EMOJIS = 1 << 3;
        const GUILD_INTEGRATIONS = 1 << 4;
        const GUILD_WEBHOOKS = 1 << 5;
        const GUILD_INVITES = 1 << 6;
        const GUILD_VOICE_STATES = 1 << 7;
        const GUILD_PRESENCES = 1 << 8;
        const GUILD_MESSAGES = 1 << 9;
        const GUILD_MESSAGE_REACTIONS = 1 << 10;
        const GUILD_MESSAGE_TYPING = 1 << 11;
        const DIRECT_MESSAGES = 1 << 12;
        const DIRECT_MESSAGE_REACTIONS = 1 << 13;
        const DIRECT_MESSAGE_TYPING = 1 << 14;
        const MESSAGE_CONTENT = 1 << 15;
        const GUILD_SCHEDULED_EVENTS = 1 << 16;
        const AUTO_MODERATION_CONFIGURATION = 1 << 17;
        const AUTO_MODERATION_EXECUTION =  1 << 18;
        const GUILD_MESSAGE_POLLS = 1 << 19;
        const DIRECT_MESSAGE_POLLS = 1 << 20;
    }
}

impl Into<u64> for Intents {
    fn into(self) -> u64 {
        self.bits()
    }
}
