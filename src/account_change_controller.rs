use std::{
  collections::HashMap,
};
use crate::{
  transaction::{
    TransactionId
  },
  page::Page, 
  utility::*, 
  journal::{
    JournalId, 
    Journal
  }, 
  unwrapper::*, 
  account::BalanceType, 
  currency::CurrencyId, 
  account_change::{
    AccountChange, 
    AccountChangeId
  }
};

pub fn new_account_change<'a>(
  currency_id: CurrencyId,
  journal_id: &Option<JournalId>,
  journals: &'a mut HashMap<JournalId, Journal>,
  page: &'a mut Page,
  transaction_id: &Option<TransactionId>
) {
  let credit_or_debit = { 
    let options = vec![
      LabeledItem::from("Debit".to_string(), BalanceType::Debit),
      LabeledItem::from("Credit".to_string(), BalanceType::Credit),
    ];

    match select_with_labels("Credit Or Debit:".to_string(), &options) {
      Err(error) => return on_error(page, error),
      Ok(credit_or_debit) => credit_or_debit.clone(),
    }
  };

  let account_id = {
    let accounts = match get_accounts(journals, journal_id) {
      None => return on_not_found(page),
      Some(accounts) => accounts,
    };

    let prompt = format!("Account:");
    let options = accounts.iter().map(
      |(id, a)| LabeledItem::from(a.name.clone(), id)
    ).collect::<Vec<_>>();
    match fuzzy_input_with_labels(prompt, &options) {
      Err(error) => return on_error(page, error),
      Ok(account_id) => *account_id,
    }
  };

  let amount = {
    let prompt = format!("Enter {} Amount:", credit_or_debit.as_str());
    let default = format!(""); 
    match input_until_parsed(prompt, default) {
      Err(error) => return on_error(page, error),
      Ok(amount) => amount,
    }
  };

  let account_change = AccountChange::new(
    account_id.clone(),
    currency_id.clone(),
    credit_or_debit,
    amount
  );

  let transaction = {
    match get_transaction_mut(journals, journal_id, transaction_id) {
      None => return on_not_found(page),
      Some(transaction) => transaction,
    }
  };

  transaction.account_changes.insert(account_change.id.clone(), account_change);
  *page = Page::SelectAccountChange;
}

pub fn select_account_change<'a>(
  account_change_id: &mut Option<AccountChangeId>,
  journal_id: &Option<JournalId>,
  journals: &'a mut HashMap<JournalId, Journal>,
  page: &'a mut Page,
  transaction_id: &Option<TransactionId>,
) {
  enum Selection {
    AccountChange(AccountChangeId),
    NewAccountChange,
    Back
  }

  let account_changes = {
    match get_account_changes(journals, journal_id, transaction_id) {
      None => return on_not_found(page),
      Some(account_changes) => account_changes,
    }
  };

  let options = {
    let mut options = account_changes.iter()
      .map(|(id, a)| {
        LabeledItem::from(
          a.lookup_name(journals, journal_id),
          Selection::AccountChange(id.clone())
        )
      })
      .collect::<Vec<_>>();
    options.sort_by(|a, b| a.label.cmp(&b.label));
    options.extend(vec![
      LabeledItem::from(
        "[New Account Change]".to_string(), 
        Selection::NewAccountChange
      ),
      LabeledItem::from("[Back]".to_string(), Selection::Back)
    ]);
    options
  };

  let selection = { 
    match select_with_labels(
      "Select Change".to_string(), 
      &options
    ) {
      Err(error) => return on_error(page, error),
      Ok(selection) => selection,
    }
  };
  
  match selection {
    Selection::AccountChange(id) => {
      match account_changes.get(id) {
        None => return on_not_found(page),
        Some(account_change) => {
          *account_change_id = Some(account_change.id.clone());
          *page = Page::ViewAccountChange;
        },
      };    
    },
    Selection::NewAccountChange => {
      *page = Page::NewAccountChange;
    },
    Selection::Back => {
      *page = Page::ViewTransaction;
    },
  };
}

pub fn view_account_change<'a>(
  account_change_id: &mut Option<AccountChangeId>,
  journal_id: &Option<JournalId>,
  journals: &'a mut HashMap<JournalId, Journal>,
  page: &'a mut Page,
  transaction_id: &Option<TransactionId>,
) {
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

  let account_change = match get_account_change(
    journals, 
    journal_id, 
    transaction_id, 
    account_change_id
  ) {
    None => return on_not_found(page),
    Some(account_change) => account_change,
  };

  let selection = { 
    match select_with_labels(
      account_change.lookup_name(journals, journal_id), 
      &options
    ) {
      Err(error) => return on_error(page, error),
      Ok(selection) => selection,
    }
  };

  match *selection {
    Selection::Display => println!("{:#?}", account_change),
    Selection::Back => {
      *page = Page::SelectTransaction
    },
    Selection::Delete => {
      *page = Page::DeleteAccountChange
    },
  }
}

pub fn delete_account_change<'a>(
  account_change_id: &Option<AccountChangeId>,
  journal_id: &Option<JournalId>,
  journals: &'a mut HashMap<JournalId, Journal>,
  page: &'a mut Page,
  transaction_id: &Option<TransactionId>,
) {
  let name = {
    match get_account_changes(
      journals, 
      journal_id, 
      transaction_id, 
    )
    .map(|account_changes| account_changes
      .get_optional(account_change_id.clone())
    )
    .flatten()
    .map(|a| a.lookup_name(journals, journal_id))
    {
      None => return on_not_found(page),
      Some(name) => name,
    }
  };

  let prompt = format!("Are you sure you want to delete \"{}\"?", name);

  let should_delete = match confirm_default(prompt) {
    Err(error) => return on_error(page, error),
    Ok(should_delete) => should_delete,
  };
  
  match should_delete {
    false => {
      *page = Page::ViewTransaction
    },
    true => {
      let account_changes = get_account_changes_mut(
        journals, 
        journal_id, 
        transaction_id
      );
      
      let id = account_change_id.clone(); 

      account_changes.map(|a|
        id.map(|id|
          a.remove(&id)
        )
      );
      *page = Page::SelectAccountChange
    },
  }
}