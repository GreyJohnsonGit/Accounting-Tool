use serde::{Serialize, Deserialize};

pub type CurrencyId = String;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Currency {
  id: CurrencyId,
  name: String,
  symbol: String
}