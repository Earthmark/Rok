use super::story;
use std::{error, fmt};

/// The representation of a pass through a story.
/// This serialized state is bound strongly to a story, but can be used as a save game of a story.
///
/// # Examples
/// ```
/// let mut telling = Telling::new(story);
/// while telling.is_running() {
///   // This can fail if the user's choice is garbage.
///   // If it fails, nothing should change in the telling.
///   telling.make_choice("word_from_user".to_string())?;
/// }
/// ```
#[derive(Clone)]
pub struct Telling<'a> {
  story: &'a story::Story,
  scene: &'a story::Scene,
  running: bool,
}

/// The introduction scene to a story didn't exist, this is a problem with the story itself.
#[derive(Debug)]
pub struct IntroSceneNotFound {}
impl fmt::Display for IntroSceneNotFound {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Intro scene not found.")
  }
}
impl error::Error for IntroSceneNotFound {}

/// The choice requested by the user did not exist for the current scene.
#[derive(Debug)]
pub struct ChoiceNotFound {
  choice: String,
}
impl fmt::Display for ChoiceNotFound {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Choice {} not found.", self.choice)
  }
}
impl error::Error for ChoiceNotFound {}

/// The destination of a scene transfer didn't exist.
#[derive(Debug)]
pub struct SceneNotFound {
  destination: String,
}
impl fmt::Display for SceneNotFound {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Scene {} not found.", self.destination)
  }
}
impl error::Error for SceneNotFound {}

impl Telling<'_> {
  /// Creates a new telling for a story, starting at the storys intro scene.
  pub fn new<'a>(story: &'a story::Story) -> Result<Telling<'a>, Box<dyn error::Error>> {
    let intro = story
      .scenes
      .get(&story.intro)
      .ok_or(IntroSceneNotFound {})?;
    Ok(Telling {
      story: story,
      scene: intro,
      running: true,
    })
  }

  /// Gets the message for the current scene of the story.
  pub fn message(&self) -> &String {
    &self.scene.message
  }

  /// Gets if the story should continue running.
  /// TODO: This should probably be an error type and not field on the telling.
  pub fn is_running(&self) -> bool {
    self.running
  }

  /// Advances the story by doing the provided choice.
  pub fn make_choice(&mut self, choice: &String) -> Result<(), Box<dyn error::Error>> {
    let chosen_choice = self.scene.choices.get(choice).ok_or(ChoiceNotFound {
      choice: choice.clone(),
    })?;
    self.do_choice(chosen_choice)
  }

  /// Actual implementation where the choice is made.
  fn do_choice(&mut self, choice: &story::Choice) -> Result<(), Box<dyn error::Error>> {
    match choice {
      story::Choice::MoveScene { destination } => {
        self.scene = self.story.scenes.get(destination).ok_or(SceneNotFound {
          destination: destination.clone(),
        })?;
      }
      story::Choice::Exit => self.running = false,
    };
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::super::story::*;
  use super::*;
  use std::collections::HashMap;

  fn make_test_story() -> Story {
    let mut scenes = HashMap::new();
    scenes.insert(
      "test_scene".to_string(),
      Scene {
        message: "test_message".to_string(),
        choices: HashMap::new(),
      },
    );
    scenes.insert(
      "other_scene".to_string(),
      Scene {
        message: "other_message".to_string(),
        choices: HashMap::new(),
      },
    );
    Story {
      intro: "potato".to_string(),
      scenes,
    }
  }

  fn make_test_telling<'a>(story: &'a Story) -> Telling<'a> {
    Telling {
      story: story,
      scene: story.scenes.get("test_scene").unwrap(),
      running: true,
    }
  }

  #[test]
  fn message_returns_from_current_scene() {
    let story = make_test_story();
    let telling = make_test_telling(&story);
    assert_eq!("test_message", telling.message());
  }

  #[test]
  fn move_scene_failure_returns_not_found() {
    let story = make_test_story();
    let mut telling = make_test_telling(&story);
    let result = telling.do_choice(&Choice::MoveScene {
      destination: "unknown_scene".to_string(),
    });
    assert!(result.is_err());
  }

  #[test]
  fn move_scene_failure_doesnt_move_scene() {
    let story = make_test_story();
    let mut telling = make_test_telling(&story);
    let initial_scene = telling.scene;
    let _ = telling.do_choice(&Choice::MoveScene {
      destination: "unknown_scene".to_string(),
    });
    assert_eq!(initial_scene, telling.scene);
  }

  #[test]
  fn move_scene_success_returns_not_found() {
    let story = make_test_story();
    let mut telling = make_test_telling(&story);
    let result = telling.do_choice(&Choice::MoveScene {
      destination: "other_scene".to_string(),
    });
    assert!(result.is_ok());
  }

  #[test]
  fn move_scene_success_failure_doesnt_move_scene() {
    let story = make_test_story();
    let mut telling = make_test_telling(&story);
    let initial_scene = telling.scene;
    let _ = telling.do_choice(&Choice::MoveScene {
      destination: "other_scene".to_string(),
    });
    assert_ne!(initial_scene, telling.scene);
  }

  #[test]
  fn exit_is_running_returns() {
    let story = make_test_story();
    let mut telling = make_test_telling(&story);
    let running_before = telling.is_running();
    let _ = telling.do_choice(&Choice::Exit);
    assert_ne!(running_before, telling.is_running());
  }
}
