use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TodoItem {
  pub id: String,
  pub title: String,
  pub description: String,
  pub status: bool,
}

impl PartialEq for TodoItem {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl TodoItem {
  pub fn new(id: String, title: String, description: String) -> TodoItem {
    return TodoItem {
      id,
      title,
      description,
      status: false,
    };
  }
}
