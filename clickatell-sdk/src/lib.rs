//! # Clickatell SDK
//! A crate for transmitting messages via [Clickatell](https://www.clickatell.com) messaging gateways.
//!
//! ```rust,no_run
//! use clickatell_sdk::one_api::{send_messages, Client, Result, Channel};
//!
//! async fn send_sms_message(api_key: &str, numbers: Vec<&str>, message: &str) -> Result<()> {
//!   let client = Client::new(api_key)?;
//!
//!   let mut request = send_messages::Request::new();
//!   for number in numbers {
//!       request.add_message(Channel::SMS, number, message)?;
//!   }
//!
//!   let response = client.send_messages(request).await?;
//!
//!   match response.error {
//!     Some(error) => eprintln!("Request Error: {:?}", error),
//!     None => {
//!       for message_response in response.messages() {
//!         match message_response.error {
//!           Some(error) => eprintln!("Message Error  {}: {}", message_response.to, error),
//!           None => println!("Message ID {}: {}", message_response.to, message_response.message_id())
//!         }
//!       }
//!     }
//!   }
//!   Ok(())
//! }
//! ```

/// Send SMS and Whatsapp message via the [Clickatell One API](https://docs.clickatell.com/channels/one-api/one-api-reference/) messaging gateweay
pub mod one_api;
