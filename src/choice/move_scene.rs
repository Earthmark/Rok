use super::super::story::Story;
use super::super::telling::Telling;
use serde::Deserialize;
use std::error;
use std::fmt;
use std::result;

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

impl error::Error for UnknownMoveDestination {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

impl MoveScene {
  pub fn make<'a>(
    &self,
    story: &'a Story,
    current: Telling<'a>,
  ) -> result::Result<Telling<'a>, Box<dyn error::Error>> {
    let scene = story
      .scenes
      .get(&self.destination)
      .ok_or(UnknownMoveDestination {
        destination: self.destination.clone(),
      })?;
    Ok(Telling { scene, ..current })
  }
}
