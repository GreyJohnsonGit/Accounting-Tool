use std::collections::HashMap;
use uuid::Uuid;
use crate::{
  account::{
    AccountId, 
    BalanceType
  }, 
  currency::CurrencyId, 
  unwrapper::get_accounts, 
  journal::{
    JournalId, 
    Journal
  }, 
  utility::error_token
};
use serde::{Serialize, Deserialize};

pub type AccountChangeId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountChange {
  pub id: AccountChangeId,
  pub account_id: AccountId,
  pub currency_id: CurrencyId,
  pub credit_or_debit: BalanceType,
  pub amount: f64,
}

impl AccountChange {
  pub fn new(
    account_id: AccountId,
    currency_id: CurrencyId,
    credit_or_debit: BalanceType,
    amount: f64
  ) -> AccountChange {
    AccountChange {
      id: Uuid::new_v4().to_string(), 
      account_id, 
      currency_id,
      credit_or_debit, 
      amount
    }
  }

  pub fn lookup_name(
    &self, 
    journals: &HashMap<JournalId, Journal>,
    journal_id: &Option<JournalId>
  ) -> String {
    let accounts = match get_accounts(journals, journal_id) {
      None => return error_token().to_string(),
      Some(accounts) => accounts,
    };

    let account_name = match accounts.get(&self.account_id) {
      None => error_token(),
      Some(account) => &account.name,
    };

    let c_or_d = match self.credit_or_debit {
      BalanceType::Debit => "D",
      BalanceType::Credit => "C",
    };

    format!("{} - {}: {}", c_or_d, account_name, self.amount)
  }
}