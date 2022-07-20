use crate::one_api::{error::Error, result::Result};
use reqwest::{
    header::HeaderMap, header::HeaderValue, header::AUTHORIZATION, Client as InnerClient,
};
use std::default::Default;

const HOSTNAME: &str = "https://platform.clickatell.com";

pub struct Client {
    api_key: String,
    hostname: String,
    inner_client: InnerClient,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn new(api_key: &str) -> Result<Self> {
        Self::builder().api_key(api_key).build()
    }
}

pub struct ClientBuilder {
    hostname: Option<String>,
    api_key: Option<String>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        ClientBuilder {
            hostname: Some(HOSTNAME.to_string()),
            api_key: None,
        }
    }
}

impl ClientBuilder {
    fn api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    fn hostname(mut self, hostname: &str) -> Self {
        self.hostname = Some(hostname.to_string());
        self
    }

    fn build(self) -> Result<Client> {
        match (self.api_key, self.hostname) {
            (Some(api_key), Some(hostname)) => {
                let mut headers = HeaderMap::new();
                let mut auth_value = HeaderValue::from_str(&api_key)?;
                auth_value.set_sensitive(true);
                headers.insert(AUTHORIZATION, auth_value);
                let inner_client = InnerClient::builder().default_headers(headers).build()?;

                Ok(Client {
                    api_key,
                    hostname,
                    inner_client,
                })
            }
            (None, _) => Err(Error::ApiKeyNotSet),
            (Some(_), None) => Err(Error::HostnameNotSet),
        }
    }
}
