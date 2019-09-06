use super::telling;
use serde::Deserialize;
use std::collections::HashMap;
use std::{error, fs, io};

use super::choice::Choice;

#[derive(Deserialize)]
pub struct Story {
  pub intro: String,
  pub scenes: HashMap<String, Scene>,
}

#[derive(Debug)]
enum RokError {
  IntroSceneNotFound,
}

impl std::fmt::Display for RokError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      RokError::IntroSceneNotFound => {
        f.write_str("Story must have a scene called 'intro' to start at.")
      }
    }
  }
}

impl error::Error for RokError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

impl Story {
  pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Story, Box<dyn error::Error>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let u = serde_json::from_reader(reader)?;
    Ok(u)
  }

  pub fn new_telling<'a>(&'a self) -> Result<telling::Telling<'a>, Box<dyn error::Error>> {
    let intro = self
      .scenes
      .get(&self.intro)
      .ok_or(RokError::IntroSceneNotFound)?;
    Ok(telling::Telling {
      scene: intro,
      running: true,
    })
  }
}

#[derive(Deserialize)]
pub struct Scene {
  pub choices: HashMap<String, Choice>,
  pub message: String,
}
