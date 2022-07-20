use std::fmt;

/// Request to retrieve the status of a previously submitted [Message][crate::one_api::send_messages::Message]
#[derive(Debug)]
pub struct Request {
  api_message_id: String,
}

impl Request {
  /// `api_message_id` is returned in a [MessageResponse][crate::one_api::send_messages::MessageResponse::message_id] from a call to [Client::send_messages][crate::one_api::Client::send_messages].
  pub fn new(api_message_id: &str) -> Self {
    Self {
      api_message_id: String::from(api_message_id),
    }
  }
}

impl fmt::Display for Request {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.api_message_id)
  }
}
