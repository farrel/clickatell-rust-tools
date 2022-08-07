use crate::one_api::send_messages::GatewayError;
use serde::Deserialize;

/// Response for a message sent to the gateway
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct MessageResponse {
  /// Unique message identifier - `None` if an error occurs processing the message,
  /// `Some` if successful
  #[serde(rename = "apiMessageId")]
  api_message_id: Option<String>,
  /// Message accepted for processing
  accepted: bool,
  /// Message destination
  pub to: String,
  /// Error preventing message being processed, `None` if successfully processed
  pub error: Option<GatewayError>,
}

impl MessageResponse {
  /// Unique message identifier
  ///
  /// If there was an error processing the message, this will return an empty string.
  pub fn message_id(&self) -> String {
    match &self.api_message_id {
      Some(message_id) => message_id.clone(),
      None => String::from(""),
    }
  }

  /// Convenience method to check if an error occured processing the message
  pub fn is_error(&self) -> bool {
    self.error.is_some()
  }
}

#[test]
fn test_deserialize() {
  use crate::one_api::send_messages;

  let succcess_202 = r#"{
      "messages": [
        {
          "apiMessageId": "02ca87d5a95446e68b0ac36a0c2057f1",
          "accepted": true,
          "to": "2799900001"
        },
        {
          "apiMessageId": "b71d513be46f4b29b16c2f354c7eef07",
          "accepted": true,
          "to": "2799900002"
        }
      ],
      "error": null
    }"#;

  let send_messages_response: send_messages::Response = serde_json::from_str(succcess_202).unwrap();
  assert_eq!(None, send_messages_response.error);

  let messages = send_messages_response.messages();
  assert_eq!(2, messages.len());

  let success_207 = r#"{
      "messages": [
        {
          "apiMessageId": "cd2d90701afe42cdb15e2670218d8567",
          "accepted": true,
          "to": "2799900001"
        },
        {
          "error": {
            "code": 23,
            "description": "Invalid or missing parameter: to ."
          },
          "accepted": false,
          "to": "27999abcde"
        }
      ],
      "error": null
    }"#;

  let send_message_response: send_messages::Response = serde_json::from_str(success_207).unwrap();
  assert_eq!(None, send_message_response.error);

  let messages = send_message_response.messages();
  assert_eq!(2, messages.len());

  let error_message = messages[1].clone();
  let error = error_message.error.unwrap();
  assert_eq!(
    send_messages::GatewayErrorCode::InvalidOrMissingParameter,
    error.code
  );

  let error_400 = r#"{
      "messages": [
        {
          "error": {
            "code": 23,
            "description": "Invalid or missing parameter: to ."
          },
          "accepted": false,
          "to": "27999abcde"
        }
      ],
      "error": null
    }"#;

  let send_message_response: send_messages::Response = serde_json::from_str(error_400).unwrap();
  assert_eq!(None, send_message_response.error);

  let messages = send_message_response.messages();
  assert_eq!(1, messages.len());

  let error_message = messages[0].clone();
  let error = error_message.error.unwrap();
  assert_eq!(
    send_messages::GatewayErrorCode::InvalidOrMissingParameter,
    error.code
  );

  let error_401 = r#"{
      "error": {
        "code": 1,
        "description": "Invalid or missing integration API Key"
      }
    }"#;

  let send_message_response: send_messages::Response = serde_json::from_str(error_401).unwrap();
  assert!(send_message_response.messages().is_empty());

  let error = send_message_response.error.unwrap();
  assert_eq!(
    send_messages::GatewayErrorCode::InvalidOrMissingIntegrationAPIKey,
    error.code
  )
}
