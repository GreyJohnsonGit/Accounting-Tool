use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::{journal::{JournalId, Journal}, currency::{CurrencyId, Currency}};

#[derive(Serialize, Deserialize)]
pub struct Data {
  pub journals: HashMap<JournalId, Journal>,
  pub currencies: HashMap<CurrencyId, Currency>
}

impl Data {
  pub fn load() -> Data {
    let path = Path::new("db.json");
    if let Ok(file) = File::open(&path) {
      serde_json::from_reader(file).unwrap()
    } else {
      let mut data = Data { 
        journals: HashMap::new(), 
        currencies: HashMap::new()
      };
      let dollars = Currency::new("Dollars".to_string(), "$".to_string());
      data.currencies.insert(dollars.id.clone(), dollars);
      data
    }
  }

  pub fn save(self) {
    let path = Path::new("db.json");
    if let Ok(file) = File::create(&path) {
      _ = serde_json::to_writer(file, &self);
    }
  }
}