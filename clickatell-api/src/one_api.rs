//! # One API
//! The Clickatell One API is a combined gateway for both SMS and Whatsapp messages.
//!
//! ```rust,ignore
//! let client = Client::new(api_key)?;
//!
//! if let Ok(status_response) = client.message_status(message_id).await? {
//!     println!("Message Status: #{status_response}")
//! }
//!
//! if let Ok(balance_response) = client.balance().await? {
//!     println!("Balance: {balance_response}");
//! }
//! ```

pub mod balance;
mod blocking_client;
mod channel;
mod client;
mod error;
pub mod message_status;
mod result;
pub mod send_messages;

pub use blocking_client::{BlockingClient, BlockingClientBuilder};
pub use channel::Channel;
pub use client::{Client, ClientBuilder};
pub use error::Error;
pub use result::Result;
