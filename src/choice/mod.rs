use super::story::Story;
use super::telling::Telling;
use serde::Deserialize;
use std::error;
use std::result;

mod exit;
mod move_scene;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Choice {
  MoveScene(move_scene::MoveScene),
  Exit(exit::Exit),
}

impl Choice {
  pub fn make<'a>(
    &self,
    story: &'a Story,
    current: Telling<'a>,
  ) -> result::Result<Telling<'a>, Box<dyn error::Error>> {
    match self {
      Choice::MoveScene(c) => c.make(story, current),
      Choice::Exit(c) => c.make(story, current),
    }
  }
}
