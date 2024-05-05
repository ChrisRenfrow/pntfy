use reqwest::{
    blocking::{Client, Response},
    Error, Url,
};
use std::{error, sync::Arc};

#[derive(Debug, Clone)]
pub struct Notifier {
    client: Arc<Client>,
    endpoint: Url,
}

impl Notifier {
    pub fn new(url: &str) -> Result<Self, Box<dyn error::Error>> {
        let endpoint: Url = url.try_into()?;
        Ok(Self {
            client: Arc::new(Client::new()),
            endpoint,
        })
    }

    pub fn notify(&self, message: &str) -> Result<Response, Error> {
        self.client
            .post(self.endpoint.clone())
            .body(message.to_owned())
            .send()
    }
}
