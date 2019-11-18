use serde::Deserialize;
use std::collections::HashMap;
use std::{error, fs, io};

/// The story to be told.
#[derive(Deserialize, PartialEq, Debug)]
pub struct Story {
  /// The name of the first scene in the story.
  pub intro: String,
  /// The scenes in the story.
  pub scenes: HashMap<String, Scene>,
}

impl Story {
  /// Loads the story from file.
  pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Story, Box<dyn error::Error>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let u = serde_json::from_reader(reader)?;
    Ok(u)
  }
}

/// A scene in the story, such as a room, or a point in a conversation.
#[derive(Deserialize, PartialEq, Debug)]
pub struct Scene {
  /// The choices that can be made in the scene.
  pub choices: HashMap<String, Choice>,
  /// A description of the scene.
  pub message: String,
}

/// The different kinds of choices available for the story.
#[derive(Deserialize, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum Choice {
  /// A choice to change the current scene to another scene.
  MoveScene { destination: String },
  /// A choice to end the story, this may not be perminant.
  Exit,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize_empty() {
    let s: Story = serde_json::from_str(r#"{ "intro": "", "scenes": {} }"#).unwrap();
    assert_eq!(s.intro, "");
    assert_eq!(s.scenes.len(), 0);
  }

  #[test]
  fn deserialize_basic_story() {
    let s: Story = serde_json::from_str(
      r#"
    {
      "intro": "intro",
      "scenes": {
        "intro": {
          "message": "Hello",
          "choices": {
            "next": {
              "type": "MoveScene",
              "destination": "outro"
            },
            "--exit": {
              "type": "Exit"
            }
          }
        },
        "outro": {
          "message": "Goodbye",
          "choices": {
            "--exit": {
              "type": "Exit"
            }
          }
        }
      }
    }"#,
    )
    .unwrap();
    let mut scenes = HashMap::new();
    let mut intro_choices = HashMap::new();
    intro_choices.insert(
      "next".to_string(),
      Choice::MoveScene {
        destination: "outro".to_string(),
      },
    );
    intro_choices.insert("--exit".to_string(), Choice::Exit);
    scenes.insert(
      "intro".to_string(),
      Scene {
        message: "Hello".to_string(),
        choices: intro_choices,
      },
    );
    let mut outro_choices = HashMap::new();
    outro_choices.insert("--exit".to_string(), Choice::Exit);
    scenes.insert(
      "outro".to_string(),
      Scene {
        message: "Goodbye".to_string(),
        choices: outro_choices,
      },
    );
    let compare = Story {
      intro: "intro".to_string(),
      scenes,
    };
    assert_eq!(s, compare);
  }
}
