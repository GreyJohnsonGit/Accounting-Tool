use std::{
  collections::HashMap,
};
use chrono::Datelike;
use dialoguer::{
  theme::ColorfulTheme, 
  Confirm
};
use crate::{
  transaction::{
    Transaction, 
    TransactionId
  },
  page::Page, 
  utility::{
    on_error, 
    on_not_found,
    LabeledItem,
    select_with_labels, 
    input_until_parsed, 
    input_default, input_until_number_with_validation, is_valid_month, is_valid_day
  }
};

pub fn new_transaction<'a>(
  page: &'a mut Page,
  transactions: Option<&'a mut HashMap<TransactionId, Transaction>>,
) {
  let transactions = match transactions {
    None => return on_not_found(page),
    Some(transactions) => transactions,
  };

  let name = {
    let prompt = format!("Transaction Name:"); 
    let default_name = format!("Transaction {}", transactions.len()); 
    match input_default(prompt, default_name) {
      Err(error) => return on_error(page, error),
      Ok(name) => name,
    }
  };

  // Build date for default inputs
  let current_date = chrono::Utc::now();
  
  let year: i32 = {
    let prompt = format!("Enter Transaction Year:");
    let default_year = format!("{}", current_date.year()); 
    match input_until_parsed(prompt, default_year) {
      Err(error) => return on_error(page, error),
      Ok(year) => year,
    }
  };
  
  let month = {
    let prompt = format!("Enter Transaction Month:");
    let default_month = format!("{}", current_date.month());
    match input_until_number_with_validation(
      prompt, 
      default_month,
      &is_valid_month
    ) {
      Err(error) => return on_error(page, error),
      Ok(month) => month,
    }
  };

  let day = {
    let prompt = format!("Enter Transaction Day:");
    let default_day = format!("{}", current_date.day());
    let validate = &|day| {
      is_valid_day(current_date.year(), current_date.month(), day)
    };
    match input_until_number_with_validation(
      prompt, 
      default_day,
      &validate
    ) {
      Err(error) => return on_error(page, error),
      Ok(day) => day,
    }
  };

  let description = {
    let prompt = format!("Enter Transaction Description");
    let default_description = format!("");
    match input_default(prompt, default_description) {
      Err(error) => return on_error(page, error),
      Ok(description) => description,
    }
  };

  let date = format!("{}/{}/{}", year, month, day);
  let transaction = Transaction::new(date, name, description);
  transactions.insert(transaction.id.clone(), transaction);
  *page = Page::SelectTransaction
}

pub fn select_transaction<'a>(
  page: &'a mut Page,
  transaction_id: &'a mut Option<TransactionId>,
  transactions: Option<&'a mut HashMap<TransactionId, Transaction>>,
) {
  enum Selection<'a> {
    Transaction(&'a TransactionId),
    NewTransaction,
    Back
  }

  let transactions = match transactions {
    None => return on_not_found(page),
    Some(transactions) => transactions,
  };

  let options = {
    let mut options = transactions.iter()
      .map(|(id, t)| {
        let label = format!("{}: {}", t.date, t.name); 
        LabeledItem::from(
          label,
          Selection::Transaction(id)
        )
      })
      .collect::<Vec<_>>();
    options.sort_by(|a, b| a.label.cmp(&b.label));
    options.extend(vec![
      LabeledItem::from(
        "[New Transaction]".to_string(), 
        Selection::NewTransaction
      ),
      LabeledItem::from("[Back]".to_string(), Selection::Back)
    ]);
    options
  };

  let selection = { 
    match select_with_labels(
      "Select Transaction".to_string(), 
      &options
    ) {
      Err(error) => return on_error(page, error),
      Ok(selection) => selection,
    }
  };
  
  match *selection {
    Selection::Transaction(id) => {
      match transactions.get(id) {
        None => return on_not_found(page),
        Some(transaction) => {
          *transaction_id = Some(transaction.id.clone());
          *page = Page::ViewTransaction;
        },
      };    
    },
    Selection::NewTransaction => {
      *page = Page::NewTransaction;
    },
    Selection::Back => {
      *page = Page::ViewJournal;
    },
  };
}

pub fn view_transaction(
  page: &mut Page,
  transaction: Option<&mut Transaction>,
) {
  enum Selection {
    Back,
    Display,
    AccountChanges,
    Delete,
  }

  let options = vec![
    LabeledItem::from("[Back]".to_string(), Selection::Back),
    LabeledItem::from("[Display]".to_string(), Selection::Display),
    LabeledItem::from("[Account Changes]".to_string(), Selection::AccountChanges),
    LabeledItem::from("[Delete]".to_string(), Selection::Delete),
  ];

  let transaction = match transaction {
    None => return on_not_found(page),
    Some(transaction) => transaction,
  };

  let selection = { 
    match select_with_labels(
      transaction.name.clone(), 
      &options
    ) {
      Err(error) => return on_error(page, error),
      Ok(selection) => selection,
    }
  };

  match *selection {
    Selection::Display => println!("{:#?}", transaction),
    Selection::Back => {
      *page = Page::SelectTransaction
    },
    Selection::Delete => {
      *page = Page::DeleteTransaction
    },
    Selection::AccountChanges => {
      *page = Page::SelectAccountChange
    },
}
}

pub fn delete_transaction<'a>(
  page: &'a mut Page,
  transactions: Option<&'a mut HashMap<TransactionId, Transaction>>,
  transaction_id: &Option<TransactionId>,
) {
  let transactions = match transactions {
    None => return on_not_found(page),
    Some(transactions) => transactions,
  };

  let transaction_id = match transaction_id {
    None => return on_not_found(page),
    Some(id) => id,
  };

  let name = match transactions.get(transaction_id) {
    None => return on_not_found(page),
    Some(transaction) => transaction.name.clone(),
  };

  let should_delete = match Confirm::with_theme(
      &ColorfulTheme::default()
    )
    .with_prompt(
      format!("Are you sure you want to delete \"{}\"?", name)
    )
    .default(false)
    .interact() 
  {
    Err(error) => return on_error(page, error),
    Ok(should_delete) => should_delete,
  };
  
  match should_delete {
    false => {
      *page = Page::ViewTransaction
    },
    true => {
      transactions.remove(transaction_id);
      *page = Page::SelectTransaction
    },
  }
}