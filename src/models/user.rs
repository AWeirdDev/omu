use bitflags::bitflags;
use ijson::IValue;
use serde::{Deserialize, Serialize};

/// Represents a user object.
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,

    /// The user's Discord-tag.
    pub discriminator: String,

    /// The user's display name, if it is set. For bots, this is the application name.
    pub global_name: Option<String>,

    /// The user's avatar hash.
    pub avatar: Option<String>,

    /// Whether the user belongs to an OAuth2 application.
    #[serde(rename = "bot")]
    pub is_bot: Option<bool>,

    /// Whether the user is an Official Discord System user (part of the privileged system role).
    #[serde(rename = "system")]
    pub is_system: Option<bool>,

    /// Whether the user has two factor enabled on their account.
    pub mfa_enabled: Option<bool>,

    /// The user's banner hash.
    pub banner: Option<String>,

    /// The user's banner color encoded as an integer representation of hexadecimal color code.
    pub accent_color: Option<u32>,

    /// The user's chosen language option. (requires `email` scope)
    pub locale: Option<String>,

    /// Whether the email on this account has been verified. (requires `email` scope)
    pub verified: Option<bool>,

    /// The flags on the user's account.
    pub flags: Option<UserFlags>,

    /// The type of Nitro subscription on the user's account.
    pub premium_type: Option<PremiumType>,

    /// The public flags on the user's account.
    pub public_flags: Option<UserFlags>,

    /// Data for the user's avatar decoration.
    pub avatar_decoration_data: Option<IValue>,
}

/// Represents the Nitro subscription type.
#[derive(Debug, Deserialize, Serialize)]
pub enum PremiumType {
    None = 1,
    NitroClassic = 2,
    Nitro = 3,
    NitroBasic = 4,
}

bitflags! {
    #[derive(Debug)]
    pub struct UserFlags: u64 {
        const STAFF = 1 << 0;
        const PARTNER = 1 << 1;
        const HYPESQUAD = 1 << 2;
        const BUG_HUNTER_LEVEL_1 = 1 << 3;
        const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;
        const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;
        const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;
        const PREMIUM_EARLY_SUPPORTER = 1 << 9;

        /// User is a team.
        const TEAM_PSEUDO_USER = 1 << 10;

        const BUG_HUNTER_LEVEL_2 = 1 << 14;
        const VERIFIED_BOT = 1 << 16;
        const VERIFIED_DEVELOPER = 1 << 17;
        const CERTIFIED_MODERATOR = 1 << 18;

        /// Bot uses only HTTP interactions and is shown in the online member list.
        const BOT_HTTP_INTERACTIONS = 1 << 19;

        const ACTIVE_DEVELOPER = 1 << 22;
    }
}

impl Into<u64> for UserFlags {
    fn into(self) -> u64 {
        self.bits()
    }
}

impl Serialize for UserFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for UserFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Ok(UserFlags::from_bits_truncate(value))
    }
}
