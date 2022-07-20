use crate::one_api::{Channel, Result};
use serde::Serialize;

/// Message sent to the gateway
#[derive(Serialize, Debug)]
pub struct Message {
  channel: Channel,
  to: String,
  content: String,
}

impl Message {
  pub fn new(channel: Channel, to: impl ToString, content: impl ToString) -> Result<Self> {
    Ok(Self {
      channel,
      to: to.to_string(),
      content: content.to_string(),
    })
  }
}
