use serde::Serialize;

/// Communication channel the message is sent on
#[derive(Serialize, Debug, PartialEq)]
pub enum Channel {
  #[serde(rename = "sms")]
  SMS,
  #[serde(rename = "whatsapp")]
  WhatsApp,
  Unknown,
}

impl std::fmt::Display for Channel {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{:?}", self)
  }
}

#[test]
fn test_channel_serialize() {
  assert_eq!(
    String::from("\"sms\""),
    serde_json::to_string(&Channel::SMS).unwrap()
  );

  assert_eq!(
    String::from("\"whatsapp\""),
    serde_json::to_string(&Channel::WhatsApp).unwrap()
  );
}
