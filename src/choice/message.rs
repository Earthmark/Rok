use super::super::story::Story;
use super::super::telling::Telling;
use serde::Deserialize;
use std::result;

type Result<'a> = result::Result<Telling<'a>, ()>;

#[derive(Deserialize)]
pub struct Message {
  message: String,
}

impl Message {
  pub fn tell<'a>(&self, _story: &'a Story, current: &Telling<'a>) -> Result<'a> {
    println!("{}", self.message);
    Ok(current.clone())
  }
}
