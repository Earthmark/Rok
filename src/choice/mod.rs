use super::story::Story;
use super::telling::Telling;
use serde::Deserialize;
use std::fmt;
use std::result;

mod message;
mod move_scene;

type Result<'a> = result::Result<Telling<'a>, ChoiceMakeError>;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Choice {
  MoveScene(move_scene::MoveScene),
  Message(message::Message),
}

#[derive(Debug)]
pub enum ChoiceMakeError {
  MoveScene(move_scene::UnknownMoveDestination),
  Message,
}

impl fmt::Display for ChoiceMakeError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ChoiceMakeError::MoveScene(e) => e.fmt(f),
      ChoiceMakeError::Message => f.write_str("Error during message write."),
    }
  }
}

impl Choice {
  pub fn make<'a>(&self, story: &'a Story, current: &Telling<'a>) -> Result<'a> {
    match self {
      Choice::MoveScene(c) => c.tell(story, current).map_err(ChoiceMakeError::MoveScene),
      Choice::Message(c) => c.tell(story, current).map_err(|_| ChoiceMakeError::Message),
    }
  }
}
