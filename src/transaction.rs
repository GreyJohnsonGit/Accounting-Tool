use {
  crate::account::{
    Credit,
    Debit
  }
};

pub type TransactionId = String;

#[derive(Debug)]
pub struct Transaction {
  id: TransactionId,
  date: String,
  title: Option<String>,
  description: Option<String>,
  credits: Vec<Credit>,
  debits: Vec<Debit>
}