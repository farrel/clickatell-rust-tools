#[derive(Debug)]
pub enum Error {
  ApiKeyNotSet,
  HostnameNotSet,
  MessageIDNotSet,
  HttpError(reqwest::Error),
  InvalidHeader(String),
  ToNotSet,
  ContentNotSet,
  TooManyMessages,
  NoMessageStatus,
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{:?}", self)
  }
}

impl From<reqwest::Error> for Error {
  fn from(error: reqwest::Error) -> Self {
    Error::HttpError(error)
  }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
  fn from(invalid_header: reqwest::header::InvalidHeaderValue) -> Self {
    Error::InvalidHeader(format!("{invalid_header}"))
  }
}
