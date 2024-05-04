use reqwest::{
    blocking::{Client, Response},
    Error, Url,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Notifier {
    client: Arc<Client>,
    endpoint: Url,
}

impl Notifier {
    pub fn new(url: &str) -> Self {
        Self {
            client: Arc::new(Client::new()),
            endpoint: url.try_into().expect("Problem parsing URL"),
        }
    }

    pub fn notify(&self, message: &str) -> Result<Response, Error> {
        self.client
            .post(self.endpoint.clone())
            .body(message.to_owned())
            .send()
    }
}
