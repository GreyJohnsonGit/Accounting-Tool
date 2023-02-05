use std::collections::HashMap;
use dialoguer::{theme::ColorfulTheme, Input, Confirm};
use crate::{
  journal::{
    Journal,
    JournalId
  },
  page::Page,
  lookup_selection::{
    lookup_selection,
    LookupEntry
  },
  utility::{
    on_error,
    on_not_found
  }
};

pub fn new_journal<'a>(
  journals: &'a mut HashMap<JournalId, Journal>, 
  page: &mut Page, 
) {
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
    Quit
  }

  let mut options = journals.iter()
    .map(|(j_id, j)| LookupEntry::from(&j.name, Selection::Journal(j_id)))
    .collect::<Vec<_>>();
  options.sort_by(|a, b| a.title.cmp(b.title));
  options.extend(vec![
    LookupEntry::from("[New Journal]", Selection::NewJournal),
    LookupEntry::from("[Quit]", Selection::Quit)
  ]);
  let options = options;

  let selection = match lookup_selection("Select Journal".to_string(), &options) {
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
        },
      };    
    },
    Selection::NewJournal => {
      *page = Page::NewJournal;
    },
    Selection::Quit => {
      *terminate_signal = true;
    },
  };
}
  
pub fn view_journal<'a>(
  journal: Option<&'a mut Journal>,
  page: &mut Page, 
) {
  enum Selection {
    Display,
    Delete,
    Back,
    Accounts,
    Transactions
  }
  
  let options = vec![
    LookupEntry::from("[Back]", Selection::Back),
    LookupEntry::from("[Accounts]", Selection::Accounts),
    LookupEntry::from("[Display]", Selection::Display),
    LookupEntry::from("[Transactions]", Selection::Transactions),
    LookupEntry::from("[Delete]", Selection::Delete),
  ];

  let journal = match journal {
    None => return on_not_found(page),
    Some(journal) => journal,
  };    

  let selection = match lookup_selection(journal.name.clone(), &options) {
      Ok(selection) => selection,
      Err(error) => return on_error(page, error),
  };
  
  match *selection {
      Selection::Display => println!("{:#?}", journal),
      Selection::Delete => {
        *page = Page::DeleteJournal;
      },
      Selection::Back => {
        *page = Page::SelectJournal;
      },
      Selection::Accounts => {
        *page = Page::SelectAccount;
      },
      Selection::Transactions => todo!(),
  }
}

pub fn delete_journal<'a>(
  journals: &'a mut HashMap<JournalId, Journal>, 
  journal_id: &Option<JournalId>,
  page: &mut Page, 
) {
  
  let journal_id = match journal_id {
    None => return on_not_found(page),
    Some(id) => id  
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
    false => {
      *page = Page::ViewJournal
    },
    true => { 
      journals.remove(journal_id); 
      *page = Page::SelectJournal;
    },
  }
}