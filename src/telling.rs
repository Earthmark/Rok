use super::story;

#[derive(Clone)]
pub struct Telling<'a> {
  pub scene: &'a story::Scene,
  pub running: bool,
}
