use std::ops::Deref;

use lexical::parse;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Snowflake {
    id: u64,
}

impl Snowflake {
    pub fn new(id: u64) -> Self {
        Snowflake { id }
    }
    
    pub fn as_u64(&self) -> u64 {
        self.id
    }

    pub fn mention_user(&self) -> String {
        format!("<@{}>", self.id.to_string())
    }

    pub fn mention_role(&self) -> String {
        format!("<@&{}>", self.id.to_string())
    }

    pub fn mention_channel(&self) -> String {
        format!("<#{}>", self.id.to_string())
    }
}

unsafe impl Send for Snowflake {}
unsafe impl Sync for Snowflake {}

impl AsRef<u64> for Snowflake {
    fn as_ref(&self) -> &u64 {
        &self.id
    }
}

impl Deref for Snowflake {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl From<Snowflake> for u64 {
    fn from(value: Snowflake) -> Self {
        value.id
    }
}

impl ToString for Snowflake {
    fn to_string(&self) -> String {
        let mut buf = itoa::Buffer::new();
        buf.format(self.id).to_string()
    }
}

impl From<String> for Snowflake {
    fn from(id: String) -> Self {
        Snowflake {
            id: parse(id.as_str()).unwrap(),
        }
    }
}

impl From<Snowflake> for String {
    fn from(id: Snowflake) -> Self {
        id.to_string()
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Snowflake {
            id: parse(s.as_str()).unwrap(),
        })
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.id.to_string())
    }
}
