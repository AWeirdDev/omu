use std::sync::Arc;

use anyhow::Result;
use reqwest::{header, Client as Reqwest};
use tokio::sync::Mutex;

use crate::dataclasses::{self, Channel, Snowflake};

use super::http_messages::CreateMessage;

#[derive(Debug)]
pub struct HttpClient {
    base: &'static str,
    client: Arc<Mutex<Reqwest>>,
}

#[derive(thiserror::Error, Debug)]
pub enum HttpError {
    #[error("http 429 rate limited")]
    RateLimited { retry_after: f32, global: bool },
}

impl From<&ijson::IValue> for HttpError {
    fn from(value: &ijson::IValue) -> Self {
        HttpError::RateLimited {
            retry_after: value["retry_after"].as_number().unwrap().to_f32().unwrap(),
            global: value["global"].to_bool().unwrap(),
        }
    }
}

impl HttpClient {
    pub fn try_new<K: ToString>(token: K) -> Result<Self> {
        let mut map = header::HeaderMap::new();
        map.append(
            header::USER_AGENT,
            header::HeaderValue::from_static("DiscordBot (https://github.com/AWeirdDev/omu)"),
        );
        map.append(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bot {}", token.to_string()))?,
        );
        let client = Reqwest::builder().default_headers(map).build()?;

        Ok(Self {
            base: "https://discord.com/api/v10",
            client: Arc::new(Mutex::new(client)),
        })
    }

    pub fn with_base(mut self, base: &'static str) -> Self {
        self.base = base;
        self
    }

    pub async fn create_message(
        &self,
        channel_id: &str,
        cm: &CreateMessage,
    ) -> Result<dataclasses::Message> {
        let client = self.client.lock().await;

        let res = client
            .post(format!("{}/channels/{}/messages", self.base, channel_id))
            .json(cm)
            .send()
            .await?;

        if res.status().as_u16() == 429 {
            let json = res.json::<ijson::IValue>().await?;
            return Err(HttpError::from(&json).into());
        }

        Ok(res.json::<dataclasses::Message>().await?)
    }

    pub async fn get_channel<T>(&self, channel_id: &Snowflake) -> Result<Channel<T>> {
        let client = self.client.lock().await;
        let res = client
            .get(format!("{}/channels/{}", self.base, channel_id.to_string()))
            .send()
            .await?;

        if res.status().as_u16() == 429 {
            let json = res.json::<ijson::IValue>().await?;
            return Err(HttpError::from(&json).into());
        }

        let channel = res.json::<Channel<T>>().await?;
        Ok(channel)
    }
}
