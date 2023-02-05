use crate::page::Page;

pub fn on_error(page: &mut Page, error: impl std::fmt::Display) {
  println!("{}", error);
  *page = Page::SelectAccount;
}

pub fn on_not_found(page: &mut Page) {
  println!("Not Found");
  *page = Page::SelectAccount;
}