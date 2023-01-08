use std::{fmt::Display, ops};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
  N,
  S,
  E,
  W,
}

impl Direction{
  pub fn next(&self)-> Direction{
    match self {
      Direction::N => Direction::S,
      Direction::S => Direction::W,
      Direction::E => Direction::N,
      Direction::W => Direction::E,
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct XY {
  pub x: i64,
  pub y: i64,
}

impl ops::Add<XY> for XY {
  type Output = XY;

  fn add(self, r: XY) -> XY {
    XY::new(self.x + r.x, self.y + r.y)
  }
}

impl ops::Add<(i64, i64)> for XY {
  type Output = XY;

  fn add(self, r: (i64, i64)) -> XY {
    XY::new(self.x + r.0, self.y + r.1)
  }
}

impl ops::Sub<XY> for XY {
  type Output = XY;

  fn sub(self, r: XY) -> Self::Output {
    XY::new(self.x - r.x, self.y - r.y)
  }
}

impl XY {
  pub fn new_usize(x: usize, y: usize) -> XY {
    XY { x: x as i64, y: y as i64 }
  }

  pub fn new(x: i64, y: i64) -> XY {
    XY { x, y }
  }

  pub fn sides(&self, dir: &Direction) -> Vec<XY> {
    match dir {
      Direction::N => vec![*self + (-1, -1), *self + (0, -1), *self + (1, -1)],
      Direction::S => vec![*self + (-1, 1), *self + (0, 1), *self + (1, 1)],
      Direction::E => vec![*self + (1, -1), *self + (1, 0), *self + (1, 1)],
      Direction::W => vec![*self + (-1, -1), *self + (-1, 0), *self + (-1, 1)],
    }
  }

  pub fn side(&self, dir:&Direction) -> XY{
    match dir {
      Direction::N => *self + (0, -1),
      Direction::S => *self + (0, 1),
      Direction::E => *self + (1, 0),
      Direction::W => *self + (-1, 0),
    }
  }

  pub fn all_sides(&self) -> Vec<XY> {
    vec![
      *self + (-1, -1),
      *self + (0, -1),
      *self + (1, -1),
      *self + (-1, 0),
      *self + (1, 0),
      *self + (-1, 1),
      *self + (0, 1),
      *self + (1, 1),
    ]
  }
}

impl Display for XY {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({},{})", self.x, self.y)
  }
}
