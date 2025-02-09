use bitflags::bitflags;

use crate::boilerplate_flags;

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
