use super::super::story::Story;
use super::super::telling::Telling;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::result;

type Result<'a> = result::Result<Telling<'a>, UnknownMoveDestination>;

#[derive(Deserialize)]
pub struct MoveScene {
  destination: String,
}

#[derive(Debug)]
pub struct UnknownMoveDestination {
  destination: String,
}

impl fmt::Display for UnknownMoveDestination {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Unknown move destination: {}", self.destination)
  }
}

impl Error for UnknownMoveDestination {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

impl MoveScene {
  pub fn tell<'a>(&self, story: &'a Story, current: &Telling<'a>) -> Result<'a> {
    story
      .scenes
      .get(&self.destination)
      .ok_or(UnknownMoveDestination {
        destination: self.destination.clone(),
      })
      .map(|scene| {
        let mut new_tell = current.clone();
        new_tell.scene = scene;
        return new_tell;
      })
  }
}
