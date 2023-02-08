use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{
  account::{Account, AccountId},
  transaction::{Transaction, TransactionId},
};

pub type JournalId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Journal {
  pub id: JournalId,
  pub name: String,
  pub accounts: HashMap<AccountId, Account>,
  pub transactions: HashMap<TransactionId, Transaction>,
}

impl Journal {
  pub fn new(name: String) -> Journal {
    let id = Uuid::new_v4().to_string();
    Journal {
      id,
      name,
      accounts: HashMap::new(),
      transactions: HashMap::new(),
    }
  }
}
