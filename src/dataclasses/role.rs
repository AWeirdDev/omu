use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::boilerplate_flags_as_u8;

use super::{HexCode, Snowflake};

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: Snowflake,
    pub name: String,
    pub color: HexCode,

    /// if this role is pinned in the user listing
    pub hoist: bool,

    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: u8,
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
    pub tags: Option<RoleTags>,
    pub flags: RoleFlags,
}

/// Tags with type null represent booleans.
/// They will be present and set to null if they are "true", and will be not present if they are "false".
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleTags {
    pub bot_id: Option<Snowflake>,
    pub integration_id: Option<Snowflake>,
    pub premium_subscriber: Option<Option<bool>>,
    pub subscription_listing_id: Option<Snowflake>,
    pub available_for_purchase: Option<Option<bool>>,
    pub guild_connections: Option<Option<bool>>,
}

bitflags! {
    #[derive(Debug)]
    pub struct RoleFlags: u8 {
        /// role can be selected by members in an onboarding prompt
        const IN_PROMPT = 1 << 0;
    }
}
boilerplate_flags_as_u8!(RoleFlags);
