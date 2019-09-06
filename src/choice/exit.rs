use super::super::story::Story;
use super::super::telling::Telling;
use serde::Deserialize;
use std::error;
use std::result;

#[derive(Deserialize)]
pub struct Exit {}

impl Exit {
  pub fn make<'a>(
    &self,
    _story: &'a Story,
    current: Telling<'a>,
  ) -> result::Result<Telling<'a>, Box<dyn error::Error>> {
    Ok(Telling {
      running: false,
      ..current
    })
  }
}
