use crate::one_api::send_messages::{GatewayError, MessageResponse};
use serde::Deserialize;

/// The response returned from a [Client::send_messages][crate::one_api::Client::send_messages] call to the Clickatell One messaging gateway.
#[derive(Deserialize, Debug)]
pub struct Response {
  /// Request level error
  ///
  /// If this is `Some` then no messages in the request were processed by the gateway.
  /// If it is `None` there may still be errors on each individual [MessageResponse] returned by
  /// [Response::messages].
  pub error: Option<GatewayError>,

  messages: Option<Vec<MessageResponse>>,
}

impl Response {
  /// Responses for each processed message
  ///
  /// If a request level error occurs then the returned [Vec] will be empty.
  pub fn messages(&self) -> Vec<MessageResponse> {
    match self.messages.clone() {
      Some(message_responses) => message_responses,
      None => Vec::new(),
    }
  }

  /// Convenience method to check if an error occured processing the corresponding
  /// [Request][crate::one_api::send_messages::Request]
  pub fn is_error(&self) -> bool {
    self.error.is_some()
  }
}
