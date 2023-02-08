use std::collections::HashMap;
use crate::{
  account::{
    Account, 
    AccountId
  },
  journal::{
    Journal, 
    JournalId
  }, 
  transaction::{
    TransactionId, 
    Transaction
  }, 
  account_change::{AccountChange, AccountChangeId},
  utility::OptionalKey
};

pub fn get_journal_mut<'a>(
  journals: &'a mut HashMap<JournalId, Journal>,
  journal_id: &Option<JournalId>,
) -> Option<&'a mut Journal> {
  journals.get_optional_mut(journal_id.clone())
}

pub fn get_journal<'a>(
  journals: &'a HashMap<String, Journal>,
  journal_id: &Option<JournalId>,
) -> Option<&'a Journal> {
  journals.get_optional(journal_id.clone())
}

pub fn get_accounts_mut<'a>(
  journals: &'a mut HashMap<String, Journal>,
  journal_id: &Option<JournalId>,
) -> Option<&'a mut HashMap<AccountId, Account>> {
  Some(&mut get_journal_mut(journals, journal_id)?.accounts)
}

pub fn get_accounts<'a>(
  journals: &'a HashMap<String, Journal>,
  journal_id: &Option<JournalId>,
) -> Option<&'a HashMap<AccountId, Account>> {
  Some(&get_journal(journals, journal_id)?.accounts)
}

pub fn get_account_mut<'a>(
  journals: &'a mut HashMap<String, Journal>,
  journal_id: &Option<JournalId>,
  account_id: &Option<AccountId>,
) -> Option<&'a mut Account> {
  get_accounts_mut(journals, journal_id)?.get_optional_mut(account_id.clone())
}

pub fn get_account<'a>(
  journals: &'a HashMap<String, Journal>,
  journal_id: &Option<JournalId>,
  account_id: &Option<AccountId>,
) -> Option<&'a Account> {
  get_accounts(journals, journal_id)?.get_optional(account_id.clone())
}

pub fn get_transactions_mut<'a>(
  journals: &'a mut HashMap<String, Journal>,
  journal_id: &Option<JournalId>,
) -> Option<&'a mut HashMap<TransactionId, Transaction>> {
  Some(&mut get_journal_mut(journals, journal_id)?.transactions)
}

pub fn get_transactions<'a>(
  journals: &'a HashMap<String, Journal>,
  journal_id: &Option<JournalId>,
) -> Option<&'a HashMap<TransactionId, Transaction>> {
  Some(&get_journal(journals, journal_id)?.transactions)
}

pub fn get_transaction_mut<'a>(
  journals: &'a mut HashMap<String, Journal>,
  journal_id: &Option<JournalId>,
  transaction_id: &Option<TransactionId>
) -> Option<&'a mut Transaction> {
  get_journal_mut(journals, journal_id)?
    .transactions
    .get_optional_mut(transaction_id.clone())
}

pub fn get_transaction<'a>(
  journals: &'a HashMap<String, Journal>,
  journal_id: &Option<JournalId>,
  transaction_id: &Option<TransactionId>
) -> Option<&'a Transaction> {
  get_journal(journals, journal_id)?
    .transactions
    .get_optional(transaction_id.clone())
}

pub fn get_account_changes_mut<'a>(
  journals: &'a mut HashMap<String, Journal>,
  journal_id: &Option<JournalId>,
  transaction_id: &Option<TransactionId>
) -> Option<&'a mut HashMap<AccountChangeId, AccountChange>> {
  Some(
    &mut get_transaction_mut(
      journals, 
      journal_id, 
      transaction_id
    )?
    .account_changes
  )
}

pub fn get_account_changes<'a>(
  journals: &'a HashMap<String, Journal>,
  journal_id: &Option<JournalId>,
  transaction_id: &Option<TransactionId>
) -> Option<&'a HashMap<AccountChangeId, AccountChange>> {
  Some(
    &get_transaction(
      journals, 
      journal_id, 
      transaction_id
    )?
    .account_changes
  )
}

pub fn get_account_change_mut<'a>(
  journals: &'a mut HashMap<String, Journal>,
  journal_id: &Option<JournalId>,
  transaction_id: &Option<TransactionId>,
  account_change_id: &Option<AccountChangeId>
) -> Option<&'a mut AccountChange> {
  get_account_changes_mut(
    journals, 
    journal_id,
    transaction_id
  )?
  .get_optional_mut(account_change_id.clone())
}

pub fn get_account_change<'a>(
  journals: &'a HashMap<String, Journal>,
  journal_id: &Option<JournalId>,
  transaction_id: &Option<TransactionId>,
  account_change_id: &Option<AccountChangeId>
) -> Option<&'a AccountChange> {
  get_account_changes(
    journals, 
    journal_id,
    transaction_id
  )?
  .get_optional(account_change_id.clone())
}