use std::{fmt::Display, ops};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
  U,
  D,
  L,
  R,
}

impl Display for Direction{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self{
        Direction::U => write!(f, "^"),
        Direction::D => write!(f, "v"),
        Direction::L => write!(f, "<"),
        Direction::R => write!(f, ">"),
    }
  }
}

#[derive(Ord, PartialOrd, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XY {
  pub y: i64,
  pub x: i64,
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

impl ops::Sub<(i64, i64)> for XY {
  type Output = XY;

  fn sub(self, r: (i64, i64)) -> Self::Output {
    XY::new(self.x - r.0, self.y - r.1)
  }
}

impl ops::Sub<&XY> for XY {
  type Output = XY;

  fn sub(self, r: &XY) -> Self::Output {
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

  pub fn manhattan(self) -> i64{
    self.x + self.y
  }

  pub fn all_sides(&self) -> Vec<XY> {
    vec![
      *self + (0, -1),
      *self + (-1, 0),
      *self + (1, 0),
      *self + (0, 1),
      *self + (0, 0), // can stay in place
    ]
  }
}

impl Display for XY {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({},{})", self.x, self.y)
  }
}
