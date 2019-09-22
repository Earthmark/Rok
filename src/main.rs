#[macro_use]
extern crate text_io;

use std::error;

mod story;
mod telling;

fn main() -> Result<(), Box<dyn error::Error>> {
  let story = story::Story::from_file("story.json")?;
  let mut state = telling::Telling::new(&story)?;
  while state.is_running() {
    println!("{}", state.message());
    let word: String = read!("{}");
    match state.make_choice(&word) {
      Ok(_) => (),
      Err(e) => println!("{}", e),
    }
  }
  Ok(())
}
