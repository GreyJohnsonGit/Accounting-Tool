use crate::{
  journal::{Journal, JournalId},
  page::Page,
  utility::{
    on_error, 
    on_not_found,
    LabeledItem,
    select_with_labels
  },
};
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use std::collections::HashMap;

pub fn new_journal<'a>(journals: &'a mut HashMap<JournalId, Journal>, page: &mut Page) {
  let name = match Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Journal Name:")
    .with_initial_text(format!("Journal {}", journals.len()))
    .interact()
  {
    Err(error) => return on_error(page, error),
    Ok(name) => name,
  };

  let journal = Journal::new(name);
  journals.insert(journal.id.clone(), journal);
  *page = Page::SelectJournal;
}

pub fn select_journal<'a>(
  journals: &'a HashMap<JournalId, Journal>,
  journal_id: &mut Option<JournalId>,
  page: &mut Page,
  terminate_signal: &mut bool,
) {
  *journal_id = None;

  enum Selection<'a> {
    Journal(&'a JournalId),
    NewJournal,
    Quit,
  }

  let mut options = journals
    .iter()
    .map(|(id, j)| LabeledItem::from(j.name.clone(), Selection::Journal(id)))
    .collect::<Vec<_>>();
  options.sort_by(|a, b| a.label.cmp(&b.label));
  options.extend(vec![
    LabeledItem::from("[New Journal]".to_string(), Selection::NewJournal),
    LabeledItem::from("[Quit]".to_string(), Selection::Quit),
  ]);
  let options = options;

  let selection = match select_with_labels("Select Journal".to_string(), &options) {
    Ok(selection) => selection,
    Err(error) => return on_error(page, error),
  };

  match *selection {
    Selection::Journal(id) => {
      match journals.get(id) {
        None => return on_not_found(page),
        Some(journal) => {
          *journal_id = Some(journal.id.clone());
          *page = Page::ViewJournal;
        }
      };
    }
    Selection::NewJournal => {
      *page = Page::NewJournal;
    }
    Selection::Quit => {
      *terminate_signal = true;
    }
  };
}

pub fn view_journal<'a>(journal: Option<&'a mut Journal>, page: &mut Page) {
  enum Selection {
    Display,
    Delete,
    Back,
    Accounts,
    Transactions,
  }

  let options = vec![
    LabeledItem::from("[Back]".to_string(), Selection::Back),
    LabeledItem::from("[Accounts]".to_string(), Selection::Accounts),
    LabeledItem::from("[Display]".to_string(), Selection::Display),
    LabeledItem::from("[Transactions]".to_string(), Selection::Transactions),
    LabeledItem::from("[Delete]".to_string(), Selection::Delete),
  ];

  let journal = match journal {
    None => return on_not_found(page),
    Some(journal) => journal,
  };

  let selection = match select_with_labels(journal.name.clone(), &options) {
    Ok(selection) => selection,
    Err(error) => return on_error(page, error),
  };

  match *selection {
    Selection::Display => println!("{:#?}", journal),
    Selection::Delete => {
      *page = Page::DeleteJournal;
    }
    Selection::Back => {
      *page = Page::SelectJournal;
    }
    Selection::Accounts => {
      *page = Page::SelectAccount;
    }
    Selection::Transactions => {
      *page = Page::SelectTransaction;
    },
  }
}

pub fn delete_journal<'a>(
  journals: &'a mut HashMap<JournalId, Journal>,
  journal_id: &Option<JournalId>,
  page: &mut Page,
) {
  let journal_id = match journal_id {
    None => return on_not_found(page),
    Some(id) => id,
  };

  let name = match journals.get(journal_id) {
    None => return on_not_found(page),
    Some(journal) => journal.name.clone(),
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
    false => *page = Page::ViewJournal,
    true => {
      journals.remove(journal_id);
      *page = Page::SelectJournal;
    }
  }
}
