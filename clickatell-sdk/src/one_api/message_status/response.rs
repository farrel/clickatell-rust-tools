use crate::one_api::message_status::status::Status;
use crate::one_api::Channel;
use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;

/// Current status of a previously submitted message
#[derive(Deserialize, PartialEq, Debug)]
pub struct Response {
  sms: Option<StatusEntry>,
  whatsapp: Option<StatusEntry>,
}

impl Response {
  /// [Channel] message was sent through
  ///
  /// [Channel::Unknown] if no valid message status returned.
  pub fn channel(&self) -> Channel {
    if self.whatsapp.is_some() {
      Channel::WhatsApp
    } else if self.sms.is_some() {
      Channel::SMS
    } else {
      Channel::Unknown
    }
  }

  fn status_entry(&self) -> StatusEntry {
    match self.channel() {
      Channel::SMS => self.sms.unwrap_or_default(),
      Channel::WhatsApp => self.whatsapp.unwrap_or_default(),
      Channel::Unknown => StatusEntry::default(),
    }
  }

  /// Status of the message
  ///
  /// [Status::Unknown] if no valid message status returned.
  pub fn status(&self) -> Status {
    self.status_entry().status
  }

  /// When the status was last updated
  ///
  /// Will be set to current UTC time if no valid message status retured.
  pub fn updated_at(&self) -> DateTime<Utc> {
    Utc.timestamp_millis(self.status_entry().timestamp)
  }
}

impl std::fmt::Display for Response {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self.status())
  }
}

#[derive(Deserialize, PartialEq, Debug, Clone, Copy)]
struct StatusEntry {
  pub status: Status,
  pub timestamp: i64,
}

impl Default for StatusEntry {
  fn default() -> Self {
    Self {
      timestamp: Utc::now().timestamp_millis(),
      status: Status::Unknown,
    }
  }
}

#[test]
fn test_deserialize() {
  let response_200 = r#" {
      "sms": {
        "status": "QUEUED",
        "timestamp": 1506607698000
      }
    }"#;

  let response = serde_json::from_str::<Response>(response_200).unwrap();

  assert_eq!(Channel::SMS, response.channel());
  assert_eq!(Status::Queued, response.status());
}
