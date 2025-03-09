use std::sync::Arc;

use crate::http::client::HttpClient;

pub(crate) trait HttpAttachable {
    fn attach(&mut self, http: Arc<HttpClient>);
}
