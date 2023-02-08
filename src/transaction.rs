use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::account_change::{AccountChange, AccountChangeId};

pub type TransactionId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
  pub id: TransactionId,
  pub date: String,
  pub name: String,
  pub description: String,
  pub account_changes: HashMap<AccountChangeId, AccountChange>,
}

impl Transaction {
  pub fn new(
    date: String,
    name: String,
    description: String,
  ) -> Transaction {
    Transaction { 
      id: Uuid::new_v4().to_string(), 
      date, 
      name, 
      description, 
      account_changes: HashMap::new(), 
    }
  }
}