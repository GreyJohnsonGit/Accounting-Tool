use dialoguer::{Select, theme::ColorfulTheme};

pub struct LookupEntry<'a, T> {
  pub title: &'a str,
  pub selection: T
}

impl <T> LookupEntry<'_, T> {
  pub fn from(title: &'_ str, selection: T) -> LookupEntry<'_, T> {
    LookupEntry {
      title,
      selection
    }
  }
}

pub fn lookup_selection<'a, T>(prompt: String, selections: &'a Vec<LookupEntry<'a, T>>) -> Result<&'a T, std::io::Error> {
  let items: Vec<&str> = selections.iter().map(|e| e.title).collect();
    
  match Select::with_theme(&ColorfulTheme::default())
    .with_prompt(prompt)
    .default(0)
    .items(&items)
    .interact() 
  {
    Err(error) => Err(error),
    Ok(i) => Ok(&selections[i].selection)
  }
}