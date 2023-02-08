use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BalanceType {
  Debit,
  Credit,
}

impl BalanceType {
  pub fn as_str(self) -> &'static str {
    match self {
      BalanceType::Debit => "Debit",
      BalanceType::Credit => "Credit",
    }
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AccountType {
  Liability,
  Equity,
  Asset,
}

pub type AccountId = String;
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
  pub id: AccountId,
  pub name: String,
  pub balance_type: BalanceType,
  pub account_type: AccountType,
  reference_count: u32
}

impl Account {
  pub fn new(name: String, balance_type: BalanceType, account_type: AccountType) -> Account {
    Account {
      id: Uuid::new_v4().to_string(),
      name,
      balance_type,
      account_type,
      reference_count: 0
    }
  }

  pub fn increment_reference(mut self) {
    self.reference_count += 1;
  }

  pub fn decrement_reference(mut self) {
    self.reference_count -= 1;
  }

  pub fn is_referenced(self) -> bool {
    self.reference_count != 0
  }
}