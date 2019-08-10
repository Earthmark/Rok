use super::story::Scene;

#[derive(Clone)]
pub struct Telling<'a> {
  pub scene: &'a Scene,
  pub running: bool,
}
