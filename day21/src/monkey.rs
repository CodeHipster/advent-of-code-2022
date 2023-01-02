#[derive(Debug)]
pub struct Monkey{
  pub id: String,
  pub action: Action,
}

#[derive(Debug)]
pub enum Action{
  Nr(i64),
  Add(String, String),
  Sub(String, String),
  Mul(String, String),
  // TODO check if we ever divide into a float.
  Div(String, String),
}