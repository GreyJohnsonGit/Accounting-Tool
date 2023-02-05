use std::collections::HashMap;

use crate::{
  journal::{
    JournalId, 
    Journal
  }, 
  account::{
    AccountId, 
    Account
  }
};

pub fn get_journal<'a>(
  journals: &'a mut HashMap<String, Journal>, 
  journal_id: &Option<JournalId>
) -> Option<&'a mut Journal> {
  match journal_id {
    None => None,
    Some(id) => match journals.get_mut(id) {
      None => None,
      Some(journal) => Some(journal),
    },
  }
}

pub fn get_accounts<'a>(
  journals: &'a mut HashMap<String, Journal>, 
  journal_id: &Option<JournalId>
) -> Option<&'a mut HashMap<AccountId, Account>> {
  match get_journal(journals, journal_id) {
    None => None,
    Some(journal) => Some(&mut journal.accounts),
}
}

pub fn get_account<'a>(
  journals: &'a mut HashMap<String, Journal>, 
  journal_id: &Option<JournalId>,
  account_id: &Option<AccountId>
) -> Option<&'a mut Account> {
  match account_id {
    None => None,
    Some(id) => match get_journal(journals, journal_id) {
      None => None,
      Some(journal) => match journal.accounts.get_mut(id) {
        None => None,
        Some(account) => Some(account),
    },
    },
  }
}