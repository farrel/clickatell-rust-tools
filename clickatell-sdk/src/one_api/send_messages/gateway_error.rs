use crate::one_api::send_messages::GatewayErrorCode;
use serde::Deserialize;

/// If a [Request][crate::one_api::send_messages::Request] or [Message][crate::one_api::send_messages::Message] is not accepted by the gateway
/// this will be returned in the corresponding [Response][crate::one_api::send_messages::Response] or [MessageResponse][crate::one_api::send_messages::Response].
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct GatewayError {
  /// Error code returned by One API
  pub code: GatewayErrorCode,
  /// Description of the error
  pub description: String,
}

impl std::fmt::Display for GatewayError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{} ({})", self.description, self.code)
  }
}
