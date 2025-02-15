use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialGuild {
    /// Unavailable as this is partial.
    pub unavailable: bool,
    pub id: String,
}
