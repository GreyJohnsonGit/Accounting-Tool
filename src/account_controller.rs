use std::collections::HashMap;

use dialoguer::{theme::ColorfulTheme, Confirm, Input};

use crate::{
  account::{Account, AccountId, AccountType, BalanceType},
  journal::Journal,
  page::Page,
  utility::{
    on_error, 
    on_not_found,
    LabeledItem,
    select_with_labels
  },
};

pub fn new_account<'a>(accounts: Option<&'a mut HashMap<AccountId, Account>>, page: &'a mut Page) {
  let accounts = match accounts {
    None => return on_not_found(page),
    Some(accounts) => accounts,
  };

  let name = match Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Account Name:")
    .with_initial_text(format!("Account {}", accounts.len()))
    .interact()
  {
    Err(error) => return on_error(page, error),
    Ok(name) => name,
  };

  let options = vec![
    LabeledItem::from("Debit".to_string(), BalanceType::Debit),
    LabeledItem::from("Credit".to_string(), BalanceType::Credit),
  ];
  let balance_type = match select_with_labels("Balance Type:".to_string(), &options) {
    Err(error) => return on_error(page, error),
    Ok(balance_type) => balance_type.clone(),
  };

  let options = vec![
    LabeledItem::from("Liability".to_string(), AccountType::Liability),
    LabeledItem::from("Asset".to_string(), AccountType::Asset),
    LabeledItem::from("Equity".to_string(), AccountType::Equity),
  ];
  let account_type = match select_with_labels("Account Type:".to_string(), &options) {
    Err(error) => return on_error(page, error),
    Ok(account_type) => account_type.clone(),
  };

  let account = Account::new(name, balance_type, account_type);
  accounts.insert(account.id.clone(), account);
  *page = Page::SelectAccount;
}

// Todo change $journal dep into $accounts dep
pub fn select_account<'a>(
  account_id: &'a mut Option<AccountId>,
  journal: Option<&'a mut Journal>,
  page: &'a mut Page,
) {
  *account_id = None;

  enum Selection<'a> {
    Account(&'a AccountId),
    NewAccount,
    Back,
  }

  let journal = match journal {
    None => return on_not_found(page),
    Some(journal) => journal,
  };

  let accounts = &journal.accounts;
  let mut options = accounts
    .iter()
    .map(|(id, a)| LabeledItem::from(a.name.clone(), Selection::Account(id)))
    .collect::<Vec<_>>();
  options.sort_by(|a, b| a.label.cmp(&b.label));
  options.extend(vec![
    LabeledItem::from("[New Account]".to_string(), Selection::NewAccount),
    LabeledItem::from("[Back]".to_string(), Selection::Back),
  ]);
  let options = options;

  let selection = match select_with_labels("Select Account".to_string(), &options) {
    Err(error) => return on_error(page, error),
    Ok(selection) => selection,
  };

  match *selection {
    Selection::Account(id) => {
      match accounts.get(id) {
        None => return on_not_found(page),
        Some(account) => {
          *account_id = Some(account.id.clone());
          *page = Page::ViewAccount;
        }
      };
    }
    Selection::NewAccount => {
      *page = Page::NewAccount;
    }
    Selection::Back => {
      *page = Page::ViewJournal;
    }
  };
}

pub fn view_account(account: Option<&mut Account>, page: &mut Page) {
  enum Selection {
    Back,
    Display,
    Delete,
  }

  let options = vec![
    LabeledItem::from("[Back]".to_string(), Selection::Back),
    LabeledItem::from("[Display]".to_string(), Selection::Display),
    LabeledItem::from("[Delete]".to_string(), Selection::Delete),
  ];

  let account = match account {
    None => return on_not_found(page),
    Some(account) => account,
  };

  let selection = match select_with_labels(account.name.to_string(), &options) {
    Err(error) => return on_error(page, error),
    Ok(selection) => selection,
  };

  match *selection {
    Selection::Display => println!("{:#?}", account),
    Selection::Back => *page = Page::SelectAccount,
    Selection::Delete => *page = Page::DeleteAccount,
  }
}

pub fn delete_account<'a>(
  accounts: Option<&'a mut HashMap<AccountId, Account>>,
  account_id: &Option<AccountId>,
  page: &'a mut Page,
) {
  let accounts = match accounts {
    None => return on_not_found(page),
    Some(accounts) => accounts,
  };

  let account_id = match account_id {
    None => return on_not_found(page),
    Some(id) => id,
  };

  let name = match accounts.get(account_id) {
    None => return on_not_found(page),
    Some(account) => account.name.clone(),
  };

  let should_delete = match Confirm::with_theme(&ColorfulTheme::default())
    .with_prompt(format!("Are you sure you want to delete \"{}\"?", name))
    .default(false)
    .interact()
  {
    Err(error) => return on_error(page, error),
    Ok(should_delete) => should_delete,
  };

  match should_delete {
    false => *page = Page::ViewAccount,
    true => {
      accounts.remove(account_id);
      *page = Page::SelectAccount
    }
  }
}
