use crate::one_api::send_messages::Message;
use crate::one_api::{Channel, Error, Result};
use serde::Serialize;

/// Request to send messages
#[derive(Serialize, Debug, Default)]
pub struct Request {
  messages: Vec<Message>,
}

impl Request {
  pub fn new() -> Self {
    Self::default()
  }

  /// Adds a new message to the request.
  ///
  /// Returns [Error::TooManyMessages] if the number of messages exceeds the gateway limit of 100
  /// messages per request.
  pub fn add_message(&mut self, channel: Channel, to: &str, content: &str) -> Result<()> {
    if self.message_count() < 100 {
      self.messages.push(Message::new(channel, to, content)?);
      Ok(())
    } else {
      Err(Error::TooManyMessages)
    }
  }

  /// Current number of messages in the request
  pub fn message_count(&self) -> usize {
    self.messages.len()
  }
}
