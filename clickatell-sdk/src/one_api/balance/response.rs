use serde::Deserialize;

/// Response returned from a [balance][crate::one_api::Client::balance] query
#[derive(Deserialize, Debug)]
pub struct Response {
  /// Three character ISO currency code
  pub currency: String,
  pub balance: f64,
}

impl std::fmt::Display for Response {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{} {}", self.currency, self.balance)
  }
}
