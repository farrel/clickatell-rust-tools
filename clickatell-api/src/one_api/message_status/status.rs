use serde::Deserialize;

/// Message status
#[derive(Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum Status {
  #[serde(rename = "UNKNOWN")]
  Unknown,
  #[serde(rename = "QUEUED")]
  Queued,
  #[serde(rename = "SCHEDULED")]
  Scheduled,
  #[serde(rename = "SENT_TO_SUPPLIER")]
  SentToSupplier,
  #[serde(rename = "DEVICE_ACK")]
  DeviceAck,
  #[serde(rename = "READ")]
  Read,
  #[serde(rename = "EXPIRED")]
  Expired,
  #[serde(rename = "STOPPED_BY_USER")]
  StoppedByUser,
  #[serde(rename = "STOPPED_BY_ADMIN")]
  StoppedByAdmin,
  #[serde(rename = "DELIVERY_FAILURE")]
  DeliveryFailure,
  #[serde(rename = "EMULATED")]
  Emulated,
  #[serde(rename = "INSUFFICIENT_ACCOUNT_BALANCE")]
  InsufficientAccountBalance,
  #[serde(rename = "VOLUME_LIMIT")]
  VolumeLimit,
  #[serde(rename = "VOLUME_LIMIT_DAILY")]
  VolumeLimitDaily,
  #[serde(rename = "VOLUME_LIMIT_MONTHLY")]
  VolumeLimitMonthly,
  #[serde(rename = "RECIPIENT_DOES_NOT_EXIST")]
  RecipientDoesNotExist,
  #[serde(rename = "ENCRYPTION_ACCESS_DENIED")]
  EncryptionAccessDenied,
  #[serde(rename = "ENCRYPTION_CONTENT_ERROR")]
  EncryptionContentError,
  #[serde(rename = "MEDIA_NOT_FOUND")]
  MediaNotFound,
  #[serde(rename = "MEDIA_SIZE_ERROR")]
  MediaSizeError,
  #[serde(rename = "MEDIA_CHECKSUM_FAILURE")]
  MediaChecksumFailure,
  #[serde(rename = "MEDIA_REJECTED_BY_SUPPLIER")]
  MediaRejectedBySupplier,
  #[serde(rename = "MEDIA_METADATA_ERROR")]
  MediaMetadataError,
  #[serde(rename = "ROUTING_ERROR")]
  RoutingError,
  #[serde(rename = "WHATSAPP_ACCOUNT_PAYMENT_ISSUE")]
  WhatsappAccountPaymentIssue,
  #[serde(rename = "WHATSAPP_RE_ENGAGEMENT_REQUIRED")]
  WhatsappReEngagementRequired,
  #[serde(rename = "WHATSAPP_SPAM_RATE_LIMIT_REACHED")]
  WhatsappSpamRateLimitReached,
  #[serde(rename = "WHATSAPP_SERVER_RATE_LIMIT")]
  WhatsappServerRateLimit,
  #[serde(rename = "WHATSAPP_HSM_NOT_AVAILABLE")]
  WhatsappHsmNotAvailable,
  #[serde(rename = "WHATSAPP_HSM_PARAM_COUNT_MISMATCH")]
  WhatsappHsmParamCountMismatch,
  #[serde(rename = "WHATSAPP_HSM_IS_MISSING")]
  WhatsappHsmIsMissing,
  #[serde(rename = "WHATSAPP_HSM_DOWNLOAD_FAILED")]
  WhatsappHsmDownloadFailed,
  #[serde(rename = "WHATSAPP_HSM_PACK_IS_MISSING")]
  WhatsappHsmPackIsMissing,
  #[serde(rename = "WHATSAPP_EXPERIMENTAL_NUMBER")]
  WhatsappExperimentalNumber,
  #[serde(rename = "WHATSAPP_TEMPLATE_TEXT_TOO_LONG")]
  WhatsappTemplateTextTooLong,
  #[serde(rename = "WHATSAPP_TEMPLATE_FORMAT_MISMATCH")]
  WhatsappTemplateFormatMismatch,
  #[serde(rename = "WHATSAPP_TEMPLATE_FORMATTING_POLICY_VIOLATED")]
  WhatsappTemplateFormattingPolicyViolated,
  #[serde(rename = "WHATSAPP_TEMPLATE_MEDIA_FORMAT_UNSUPPORTED")]
  WhatsappTemplateMediaFormatUnsupported,
  #[serde(rename = "WHATSAPP_PARAMETER_MISSING")]
  WhatsappParameterMissing,
  #[serde(rename = "WHATSAPP_PARAMETER_INVALID")]
  WhatsappParameterInvalid,
  #[serde(rename = "WHATSAPP_PARAMETER_NOT_REQUIRED")]
  WhatsappParameterNotRequired,
  #[serde(rename = "WHATSAPP_TEMPLATE_INVALID_URL")]
  WhatsappTemplateInvalidUrl,
  #[serde(rename = "WHATSAPP_TEMPLATE_INVALID_PHONE_NUMBER")]
  WhatsappTemplateInvalidPhoneNumber,
  #[serde(rename = "WHATSAPP_TEMPLATE_RECEIVER_NO_BUTTON_SUPPORT")]
  WhatsappTemplateReceiverNoButtonSupport,
}

impl Default for Status {
  fn default() -> Self {
    Self::Unknown
  }
}
