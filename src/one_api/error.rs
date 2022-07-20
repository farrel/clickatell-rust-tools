pub enum Error {
    ApiKeyNotSet,
    HostnameNotSet,
    ReqwestError(reqwest::Error),
    InvalidHeader(String),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(invalid_header: reqwest::header::InvalidHeaderValue) -> Self {
        Error::InvalidHeader(format!("{invalid_header}"))
    }
}
