use crate::one_api::{balance, message_status, send_messages};
use crate::one_api::{error::Error, result::Result};
use reqwest::{
    header::HeaderMap, header::HeaderValue, header::ACCEPT, header::AUTHORIZATION,
    header::CONTENT_TYPE, Client as HTTPClient,
};
use std::default::Default;

pub const HOSTNAME: &str = "https://platform.clickatell.com";
const MESSAGE_PATH: &str = "/v1/message";
const BALANCE_PATH: &str = "/v1/balance";
const APPLICATION_JSON: &str = "application/json";

/// Clickatell One messaging gateway client
pub struct Client {
    hostname: String,
    http_client: HTTPClient,
}

impl Client {
    pub fn new(api_key: &str) -> Result<Self> {
        Self::builder().api_key(api_key).build()
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Send messages via the gateway
    ///
    /// A return of `Ok` does not mean the messages have been successfully delivered, only that
    /// the messages have been accepted by the gateway. Message status can be queried with
    /// [message_status].
    ///
    /// ```rust,ignore
    /// let mut request = send_messages::Request::new();
    /// request.add_message(Channel::SMS, to, "This is an SMS message");
    /// request.add_message(Channel::WhatsApp, to, "This is a WhatsApp message");
    ///
    /// let response = client.send_messages(request).await?;
    /// for message_response in response.messages() {
    ///     println!("{} {}", message_response.to, message_response.message_id());
    /// }
    /// ```
    pub async fn send_messages(
        &self,
        send_messages_request: send_messages::Request,
    ) -> Result<send_messages::Response> {
        if send_messages_request.message_count() >= 100 {
            return Err(Error::TooManyMessages);
        }

        let response = self
            .http_client
            .post(format!("{}{MESSAGE_PATH}", self.hostname))
            .json(&send_messages_request)
            .send()
            .await?;

        Ok(response.json::<send_messages::Response>().await?)
    }

    /// Query the delivery status of a message
    ///
    /// ```rust,ignore
    /// let request = message_status::Request::new(message_id);
    /// let status_response = client.message_status(request).await?;
    /// println!("Status: {}", status_response.status())
    /// ```
    pub async fn message_status(
        &self,
        request: message_status::Request,
    ) -> Result<message_status::Response> {
        let response = self
            .http_client
            .get(format!("{}{MESSAGE_PATH}/{}", self.hostname, request))
            .send()
            .await?;

        Ok(response.json::<message_status::Response>().await?)
    }

    /// Return the balance of your Clickatell account
    ///
    /// ```rust,ignore
    /// let balance_response = client.balance().await?;
    /// println!("Your balance is {} {}", balance_response.currency, balance_response.balance);
    /// ```
    pub async fn balance(&self) -> Result<balance::Response> {
        let response = self
            .http_client
            .get(format!("{}{BALANCE_PATH}", self.hostname))
            .send()
            .await?;

        Ok(response.json::<balance::Response>().await?)
    }
}

/// Used to create a [Client] that can point to a different URL than the default Clickatell One
/// gateway 
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
    pub fn api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    pub fn hostname(mut self, hostname: &str) -> Self {
        self.hostname = Some(hostname.to_string());
        self
    }

    pub fn build(self) -> Result<Client> {
        match (self.api_key, self.hostname) {
            (Some(api_key), Some(hostname)) => {
                let mut headers = HeaderMap::new();

                let mut auth_value = HeaderValue::from_str(&api_key)?;
                auth_value.set_sensitive(true);

                headers.insert(AUTHORIZATION, auth_value);
                headers.insert(CONTENT_TYPE, HeaderValue::from_str(APPLICATION_JSON)?);
                headers.insert(ACCEPT, HeaderValue::from_str(APPLICATION_JSON)?);

                let http_client = HTTPClient::builder().default_headers(headers).build()?;

                Ok(Client {
                    hostname,
                    http_client,
                })
            }
            (None, _) => Err(Error::ApiKeyNotSet),
            (Some(_), None) => Err(Error::HostnameNotSet),
        }
    }
}
