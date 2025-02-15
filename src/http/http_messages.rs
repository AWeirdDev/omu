use std::sync::Arc;

use anyhow::Result;
use serde::Serialize;

use crate::dataclasses::{Embed, Message, MessageReference, Nounce};

use super::client::HttpClient;

#[derive(Debug, Serialize)]
pub struct CreateMessage {
    pub content: Option<String>,
    pub nounce: Option<Nounce>,
    pub tts: Option<bool>,
    pub embeds: Option<Vec<Embed>>,
    pub allowed_mentions: Option<Vec<String>>,
    pub message_reference: Option<MessageReference>,
}

pub struct PrepareCreateMessageBuilder<'a> {
    cm: CreateMessage,
    http: &'a Arc<HttpClient>,
    channel_id: &'a String,
}

impl<'a> PrepareCreateMessageBuilder<'a> {
    pub(crate) fn new(http: &'a Arc<HttpClient>, channel_id: &'a String) -> Self {
        Self {
            cm: CreateMessage {
                content: None,
                nounce: None,
                tts: None,
                embeds: None,
                allowed_mentions: None,
                message_reference: None,
            },
            http,
            channel_id,
        }
    }

    pub fn content(mut self, content: String) -> Self {
        self.cm.content = Some(content);
        self
    }

    pub fn nounce(mut self, nounce: Nounce) -> Self {
        self.cm.nounce = Some(nounce);
        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.cm.tts = Some(tts);
        self
    }

    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.cm.embeds = Some(embeds);
        self
    }

    pub fn allowed_mentions(mut self, allowed_mentions: Vec<String>) -> Self {
        self.cm.allowed_mentions = Some(allowed_mentions);
        self
    }

    pub fn message_reference(mut self, message_reference: MessageReference) -> Self {
        self.cm.message_reference = Some(message_reference);
        self
    }

    pub async fn send(&self) -> Result<Message> {
        self.http.create_message(&self.channel_id, &self.cm).await
    }
}
