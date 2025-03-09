use std::sync::Arc;

use crate::http::client::HttpClient;

pub trait HttpAttachable {
    fn attach(&mut self, http: Arc<HttpClient>);
}

pub trait Mentionable {
    fn mention(&self) -> String;
}
