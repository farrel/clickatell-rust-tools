use serde_repr::Deserialize_repr;

/// Error codes returnd in a
/// [GatewayError][crate::one_api::send_messages::GatewayErrorCode]
#[derive(Deserialize_repr, PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum GatewayErrorCode {
  InvalidOrMissingIntegrationAPIKey = 1,
  AccountIsNotActive = 2,
  IntegrationIsNotActive = 3,
  OriginatingIPAddressIsNotApprovedInYourAccount = 7,
  InternalError = 18,
  InternalErrorPleaseRetry = 19,
  InsufficientAccountBalance = 20,
  PayloadDataIsMalformed = 21,
  MaximumMessagesPerRequestPayloadExceeded = 22,
  InvalidOrMissingParameter = 23,
  MaximumMessageContentSizeExceeded = 24,
  InvalidRecipientAddress = 25,
  RecipientOptedOut = 26,
  RecipientNotAvailableOnChannel = 27,
  RecipientNotAvailableOnSandbox = 28,
  ContentTypeNotSupported = 30,
  MediaFileSizeExceedsLimit = 31,
  MediaPayloadSizeExceedsLimit = 32,
  MediaItemNotFound = 33,
  ChannelFeatureIsNotActiveOnIntegration = 38,
  ChannelIsNotAvailableOnIntegration = 39,
  CharacterSetIsNotsupported = 40,
  ResourceDoesNotExist = 41,
  HttpMethodIsNotSupportedOnThisResource = 42,
  RateLimit = 43,
  FromNumberIsSuspended = 44,
  FromNumberIsNotRelatedToIntegration = 45,
  DemoAccessHasExpired = 46,
  MaximumMessagePartsExceeded = 100,
  DestinationDoesNotSupportTwoWayMessaging = 101,
  UsaCountryLimitMustUseTwoWayIntegration = 110,
  UsaCountryLimitMustEnableStopSUbscribeCommandsOnIntegration = 111,
}

impl std::fmt::Display for GatewayErrorCode {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}", *self as u8)
  }
}

#[test]
fn test_display() {
  assert_eq!(
    "1",
    format!("{}", GatewayErrorCode::InvalidOrMissingIntegrationAPIKey)
  );
}
