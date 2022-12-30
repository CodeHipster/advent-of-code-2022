
pub struct Monkey{
  pub id: String,
  pub action: Action,
}

pub enum Action{
  Nr(u16),
  Add(String, String),
  Sub(String, String),
  Mul(String, String),
  // TODO check if we ever divide into a float.
  Div(String, String),
}