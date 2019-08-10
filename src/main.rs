#[macro_use]
extern crate text_io;

use std::{error, fmt};

mod choice;
mod story;
mod telling;

#[derive(Debug)]
pub enum RokError {
  StoryStartup(story::StoryLoadError),
  IntroSceneNotFound,
}

impl fmt::Display for RokError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      RokError::StoryStartup(e) => e.fmt(f),
      RokError::IntroSceneNotFound => {
        f.write_str("Story must have a scene called 'intro' to start at.")
      }
    }
  }
}

impl error::Error for RokError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match self {
      RokError::StoryStartup(e) => Some(e),
      RokError::IntroSceneNotFound => None,
    }
  }
}

fn main() {
  let story = story::Story::from_file("story.json");
  match story
    .or_else(|e| Err(RokError::StoryStartup(e)))
    .and_then(|story| {
      story
        .scenes
        .get("intro")
        .map(|scene| telling::Telling {
          scene,
          running: true,
        })
        .ok_or(RokError::IntroSceneNotFound)
        .and_then(|telling| run_story(&story, telling))
    }) {
    Ok(_) => println!("Goodbye."),
    Err(e) => println!("{}", e),
  }
}

fn run_story<'a>(story: &'a story::Story, telling: telling::Telling<'a>) -> Result<(), RokError> {
  let mut state = telling;
  while state.running {
    let word: String = read!();
    if word == "--exit" {
      state.running = false;
      continue;
    }

    let choice = state.scene.choices.get(&word);
    match choice {
      Some(choice) => match choice.make(story, &state) {
        Ok(new_state) => state = new_state,
        Err(e) => println!("Error: {}", e),
      },
      None => println!("Unknown verb, try {:?}", state.scene.choices.keys()),
    }
  }
  return Ok(());
}
