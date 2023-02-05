use std::collections::HashMap;
use lib::{
  journal::{Journal, JournalId},
  page::Page,
  journal_controller::*,
  account_controller::*, 
  account::AccountId, unwrapper::{get_journal, get_accounts, get_account}
};

fn main() {
  let mut journals: HashMap<String, Journal> = HashMap::new();
  let mut page = Page::SelectJournal;
  let mut journal_id: Option<JournalId> = None;
  let mut account_id: Option<AccountId> = None;
  let mut terminate_signal = false;
  while !terminate_signal {
    match page {
      // Journal Pages
      Page::SelectJournal => select_journal(
        &journals, 
        &mut journal_id,
        &mut page, 
        &mut terminate_signal, 
      ),
      Page::NewJournal => new_journal(
        &mut journals,
        &mut page,
      ),
      Page::ViewJournal => view_journal(
        get_journal(&mut journals, &journal_id),
        &mut page, 
      ),
      Page::DeleteJournal => delete_journal(
        &mut journals,
        &journal_id,
        &mut page
      ),

      // Account Pages
      Page::SelectAccount => select_account(
        &mut account_id,
        get_journal(&mut journals, &journal_id),
        &mut page
      ),
      Page::NewAccount => new_account(
        get_accounts(&mut journals, &journal_id),
        &mut page
      ),
      Page::ViewAccount => view_account(
        get_account(&mut journals, &journal_id, &account_id),
        &mut page
      ),
      Page::DeleteAccount => delete_account(
        get_accounts(&mut journals, &journal_id),
        &account_id,
        &mut page
      ),
    };
  }
}
  
