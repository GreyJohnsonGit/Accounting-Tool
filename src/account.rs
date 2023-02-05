use uuid::Uuid;

use crate::{
  currency::CurrencyId,
};

#[derive(Debug, Clone, Copy)]
pub enum BalanceType {
  Debit,
  Credit
}

#[derive(Debug, Clone, Copy)]
pub enum AccountType {
  Liability,
  Equity,
  Asset
}

pub type AccountId = String;
#[derive(Debug)]
pub struct Account {
  pub id: AccountId,
  pub name: String,
  pub balance_type: BalanceType,
  pub account_type: AccountType
}

impl Account {
  pub fn new(
    name: String,
    balance_type: BalanceType,
    account_type: AccountType
  ) -> Account {
    Account { 
      id: Uuid::new_v4().to_string(),
      name,
      balance_type,
      account_type
    }
  }
}

pub trait AccountChange {
  fn get_account_id(&self) -> AccountId;
  fn get_currency_id(&self) -> CurrencyId;
  fn get_value(&self) -> f64;
}

#[derive(Debug)]
pub struct Debit {
  account_id: AccountId,
  currency_id: CurrencyId,
  amount: f64,
}

#[derive(Debug)]
pub struct Credit {
  account_id: AccountId,
  currency_id: CurrencyId,
  amount: f64,
}