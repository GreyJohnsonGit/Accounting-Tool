use lib::{
  account::AccountId,
  account_controller::*,
  journal::{Journal, JournalId},
  journal_controller::*,
  transaction_controller::*,
  page::Page,
  unwrapper::*, 
  transaction::TransactionId,
  currency::{
    Currency, 
    CurrencyId}, 
    account_change_controller::*, 
    account_change::AccountChangeId, 
    data::Data,
};
use std::collections::HashMap;

fn main() {
  let data = Data::load();
  let mut journals: HashMap<JournalId, Journal> = data.journals;
  let currencies: HashMap<CurrencyId, Currency> = data.currencies;
  let currency_id = currencies.iter().last().unwrap().0;

  let mut page = Page::SelectJournal;
  let mut journal_id: Option<JournalId> = None;
  let mut account_id: Option<AccountId> = None;
  let mut transaction_id: Option<TransactionId> = None;
  let mut account_change_id: Option<AccountChangeId> = None;
  let mut terminate_signal = false;
  while !terminate_signal {
    match page {
      // Journal Pages
      Page::SelectJournal => select_journal(
        &journals, 
        &mut journal_id, 
        &mut page, 
        &mut terminate_signal
      ),
      Page::NewJournal => new_journal(
        &mut journals, 
        &mut page
      ),
      Page::ViewJournal => view_journal(
        get_journal_mut(&mut journals, &journal_id), &mut page
      ),
      Page::DeleteJournal => delete_journal(
        &mut journals, 
        &journal_id, 
        &mut page
      ),

      // Account Pages
      Page::SelectAccount => select_account(
        &mut account_id,
        get_journal_mut(&mut journals, &journal_id),
        &mut page,
      ),
      Page::NewAccount => new_account(
        get_accounts_mut(&mut journals, &journal_id), &mut page
      ),
      Page::ViewAccount => view_account(
        get_account_mut(&mut journals, &journal_id, &account_id),
        &mut page,
      ),
      Page::DeleteAccount => delete_account(
        get_accounts_mut(&mut journals, &journal_id),
        &account_id,
        &mut page,
      ),

      // Transaction Pages
      Page::SelectTransaction => select_transaction(
        &mut page, 
        &mut transaction_id, 
        get_transactions_mut(&mut journals, &journal_id)
      ),
      Page::NewTransaction => new_transaction(
        &mut page, 
        get_transactions_mut(&mut journals, &journal_id)
      ),
      Page::ViewTransaction => view_transaction(
        &mut page, 
        get_transaction_mut(&mut journals, &journal_id, &transaction_id)
      ),
      Page::DeleteTransaction => delete_transaction(
        &mut page,
        get_transactions_mut(&mut journals, &journal_id), 
        &mut transaction_id, 
      ),

      // Debits/Credits Pages
      Page::SelectAccountChange => select_account_change(
        &mut account_change_id, 
        &journal_id,
        &mut journals, 
        &mut page, 
        &transaction_id
      ),
      Page::NewAccountChange => new_account_change(
        currency_id.clone(), 
        &journal_id, 
        &mut journals, 
        &mut page, 
        &transaction_id
      ),
      Page::ViewAccountChange => view_account_change(
        &mut account_change_id, 
        &journal_id, 
        &mut journals, 
        &mut page,
        &transaction_id),
      Page::DeleteAccountChange => delete_account_change(
        &account_change_id, 
        &journal_id, 
        &mut journals, 
        &mut page, 
        &transaction_id),
    };
  }
  Data {
    journals,
    currencies
  }.save();
}
