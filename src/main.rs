#[macro_use]
extern crate text_io;

use std::error;

mod choice;
mod story;
mod telling;

fn main() -> Result<(), Box<dyn error::Error>> {
  let story = story::Story::from_file("story.json")?;
  let mut state = story.new_telling()?;
  while state.running {
    state = run_scene(&story, state)?;
  }
  Ok(())
}

fn run_scene<'a>(
  story: &'a story::Story,
  state: telling::Telling<'a>,
) -> Result<telling::Telling<'a>, Box<dyn error::Error>> {
  println!("{}", state.scene.message);
  let word: String = read!();
  let choice = state.scene.choices.get(&word);
  match choice {
    Some(choice) => choice.make(story, state),
    None => {
      println!("Unknown verb, try {:?}", state.scene.choices.keys());
      Ok(state.clone())
    }
  }
}
