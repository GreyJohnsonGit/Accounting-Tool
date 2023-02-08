use chrono::NaiveDate;
use dialoguer::{
  Input, 
  theme::ColorfulTheme, 
  Select,
  FuzzySelect, Confirm
};
use crate::page::Page;
use std::{io::Error, str::FromStr, collections::HashMap, hash::Hash};

pub fn error_token() -> &'static str {
  "[ERROR]"
}

pub fn on_error(page: &mut Page, error: impl std::fmt::Display) {
  println!("{}", error);
  *page = Page::SelectAccount;
}

pub fn on_not_found(page: &mut Page) {
  println!("Not Found");
  *page = Page::SelectAccount;
}

pub fn is_valid_month(month: u32) -> bool {
  month > 0 && month <= 12
}

pub fn is_valid_day(year: i32, month: u32, day: u32) -> bool {
  NaiveDate::from_ymd_opt(year, month, day).is_some()
}

pub fn fuzzy_input_with_labels<'a, T>(
  prompt: String,
  items: &'a Vec<LabeledItem<T>>,
) -> Result<&'a T, Error> {
  let labels: Vec<_> = items.iter().map(|e| &e.label).collect();

  match FuzzySelect::with_theme(&ColorfulTheme::default())
    .with_prompt(prompt)
    .default(0)
    .items(&labels)
    .interact()
  {
    Err(error) => Err(error),
    Ok(i) => Ok(&items[i].item),
  }
}

pub fn confirm_default(
  prompt: String
) -> Result<bool, Error> {
  Confirm::with_theme(&ColorfulTheme::default())
    .with_prompt(prompt)
    .default(false)
    .interact() 
}

pub fn input_default(
  prompt: String,
  default_text: String
) -> Result<String, Error> {
  Input::with_theme(&ColorfulTheme::default())
    .with_prompt(prompt)
    .with_initial_text(default_text)
    .interact_text() 
}

pub fn input_until_parsed<F> (
  prompt: String,
  default_input: String
) -> Result<F, Error> where F: FromStr + Copy  {
  loop {
    let string_representation: String = match 
      Input::with_theme(&ColorfulTheme::default())
      .with_prompt(prompt.clone())
      .with_initial_text(default_input.clone())
      .interact_text()
    {
      Err(error) => return Err(error),
      Ok(string_representation) => string_representation,
    };

    if let Some(number) = string_representation.parse().ok() {
      return Ok(number);
    }
  }
}

pub fn input_until_number_with_validation<F>(
  prompt: String,
  default_input: String,
  validate: &dyn Fn(F)-> bool
) -> Result<F, Error> where F: FromStr + Copy {
  loop {
    let number = match input_until_parsed(
      prompt.clone(), 
      default_input.clone()
    ) {
      Err(error) => return Err(error),
      Ok(number) => number
    };

    if validate(number) {
      return Ok(number);
    }
  }
}
pub struct LabeledItem<T> {
  pub label: String,
  pub item: T,
}

impl<T> LabeledItem<T> {
  pub fn from(label: String, item: T) -> LabeledItem<T> {
    LabeledItem { label, item }
  }
}

pub fn select_with_labels<'a, T>(
  prompt: String,
  items: &'a Vec<LabeledItem<T>>,
) -> Result<&'a T, Error> {
  let labels: Vec<_> = items.iter().map(|e| &e.label).collect();

  match Select::with_theme(&ColorfulTheme::default())
    .with_prompt(prompt)
    .default(0)
    .items(&labels)
    .interact()
  {
    Err(error) => Err(error),
    Ok(i) => Ok(&items[i].item),
  }
}

pub trait OptionalKey<'a, K, V> { 
  fn get_optional(&'a self, key: Option<K>) -> Option<&'a V>;
  fn get_optional_mut(&'a mut self, key: Option<K>) -> Option<&'a mut V>;
}

impl <'a, K, V> OptionalKey<'a, K, V> for HashMap<K, V> where K: Eq + Hash + 'a {
  fn get_optional(&'a self, key: Option<K>) -> Option<&'a V> {
    key.map(|k| self.get(&k)).flatten()
  }

  fn get_optional_mut(&'a mut self, key: Option<K>) -> Option<&'a mut V> {
    key.map(|k| self.get_mut(&k)).flatten()
  }
}