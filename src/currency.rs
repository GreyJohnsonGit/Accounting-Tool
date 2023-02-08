use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type CurrencyId = String;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Currency {
  pub id: CurrencyId,
  pub name: String,
  pub symbol: String,
}

impl Currency {
  pub fn new(name: String, symbol: String) -> Currency {
    Currency {
      id: Uuid::new_v4().to_string(),
      name,
      symbol,
    }
  }
}