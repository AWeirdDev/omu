use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::boilerplate_flags;

use super::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub id: Snowflake,
    pub filename: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: usize,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<usize>,
    pub width: Option<usize>,

    /// Ephemeral attachments will automatically be removed after a set period of time.
    /// Ephemeral attachments on messages are guaranteed to be available as long as the message itself exists.
    pub ephemeral: Option<usize>,

    pub duration_secs: Option<f32>,
    pub waveform: Option<String>,
    pub flags: Option<AttachmentFlags>,
}

bitflags! {
    #[derive(Debug)]
    pub struct AttachmentFlags: u64 {
        const IS_REMIX = 1 << 2;
    }
}
boilerplate_flags!(AttachmentFlags);
