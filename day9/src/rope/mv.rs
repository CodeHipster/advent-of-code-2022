use std::collections::HashMap;

use nalgebra::Vector2;

use crate::rope::Knot;

#[derive(Debug)]
pub struct Move<'a> {
    pub tr: &'a Knot, //translation
    pub n: u8,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Dir {
    U,
    D,
    L,
    R,
}

// the translations a knot can make.
pub fn translations() -> HashMap<Dir, Vector2<i32>>{
  HashMap::from([
    (Dir::U, Vector2::new(0, 1)),
    (Dir::D, Vector2::new(0, -1)),
    (Dir::L, Vector2::new(-1, 0)),
    (Dir::R, Vector2::new(1, 0))
  ])
}
