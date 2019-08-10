use serde::Deserialize;
use std::collections::HashMap;
use std::{error, fmt, fs, io};

use super::choice::Choice;

#[derive(Deserialize)]
pub struct Story {
  pub scenes: HashMap<String, Scene>,
}

#[derive(Debug)]
pub enum StoryLoadError {
  File(io::Error),
  Deserialize(serde_json::Error),
}

impl fmt::Display for StoryLoadError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      StoryLoadError::File(e) => e.fmt(f),
      StoryLoadError::Deserialize(e) => e.fmt(f),
    }
  }
}

impl error::Error for StoryLoadError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match self {
      StoryLoadError::File(e) => Some(e),
      StoryLoadError::Deserialize(e) => Some(e),
    }
  }
}

impl From<io::Error> for StoryLoadError {
  fn from(err: io::Error) -> StoryLoadError {
    StoryLoadError::File(err)
  }
}
impl From<serde_json::Error> for StoryLoadError {
  fn from(err: serde_json::Error) -> StoryLoadError {
    StoryLoadError::Deserialize(err)
  }
}

impl Story {
  pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Story, StoryLoadError> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let u = serde_json::from_reader(reader)?;
    Result::Ok(u)
  }
}

#[derive(Deserialize)]
pub struct Scene {
  pub choices: HashMap<String, Choice>,
  pub arrive: Option<Choice>,
  pub depart: Option<Choice>,
}
