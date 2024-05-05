use reqwest::{
    blocking::{Client, Response},
    header::HeaderMap,
    Error, Url,
};
use std::{error, sync::Arc};

use crate::cli::Cli;

#[derive(Debug, Clone)]
pub struct Notifier {
    client: Arc<Client>,
    endpoint: Url,
    #[allow(dead_code)]
    options: Option<NtfyOptions>,
    headers: HeaderMap,
}

impl Notifier {
    pub fn new(url: &str, options: Option<NtfyOptions>) -> Result<Self, Box<dyn error::Error>> {
        let mut headers = HeaderMap::new();

        if let Some(options) = &options {
            if !options.firebase {
                headers.append("X-Firebase", "no".parse().unwrap());
            }
            if !options.caching {
                headers.append("X-Caching", "no".parse().unwrap());
            }
        }

        let endpoint: Url = url.try_into()?;

        Ok(Self {
            client: Arc::new(Client::new()),
            endpoint,
            options,
            headers,
        })
    }

    pub fn notify(&self, message: &str) -> Result<Response, Error> {
        self.client
            .post(self.endpoint.clone())
            .headers(self.headers.clone())
            .body(message.to_owned())
            .send()
    }
}

#[derive(Debug, Clone)]
pub struct NtfyOptions {
    /// Messages by default are stored on the ntfy.sh servers for 12-hours for
    /// redelivery to disconnected subscribers, turning this off means only
    /// connected subscribers will receive notifications.
    /// https://docs.ntfy.sh/publish/#message-caching
    caching: bool,
    /// By default, the ntfy server forwards messages to Firebase to leverage
    /// Firebase Cloud Messaging benefits. Disabling this means messages may
    /// be delayed (up to 15-minutes), but only touch the ntfy server.
    /// https://docs.ntfy.sh/publish/#disable-firebase
    firebase: bool,
}

impl From<Cli> for NtfyOptions {
    fn from(source: Cli) -> Self {
        Self {
            caching: !source.no_cache,
            firebase: !source.no_firebase,
        }
    }
}
