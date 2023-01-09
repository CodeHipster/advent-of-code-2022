use crate::twod::{Direction, XY};

pub struct Blizzard {
  pub pos: XY,
  pub dir: Direction,
}


impl Blizzard {
  pub fn new(pos: XY, dir: Direction) -> Blizzard {
    Blizzard { pos, dir }
  }

  pub fn shift(&self, walls: (XY, XY)) -> Blizzard{
    match self.dir {
      Direction::U => {
        let mut pos = self.pos + (0, -1);
        if pos.y <= walls.0.y {
          pos = XY { x: self.pos.x, y: walls.1.y - 1 };
        }
        return Blizzard{pos, dir: self.dir}
      }
      Direction::D => {
        let mut pos = self.pos + (0, 1);
        if pos.y >= walls.1.y {
          pos = XY { x: self.pos.x, y: walls.0.y + 1 };
        }
        return Blizzard{pos, dir: self.dir}        
      },
      Direction::L => {
        let mut pos = self.pos + (-1, 0);
        if pos.x <= walls.0.x {
          pos = XY { x: walls.1.x-1, y: self.pos.y };
        }
        return Blizzard{pos, dir: self.dir}
      },
      Direction::R => {
        let mut pos = self.pos + (1, 0);
        if pos.x >= walls.1.x {
          pos = XY { x: walls.0.x+1, y: self.pos.y };
        }
        return Blizzard{pos, dir: self.dir}
      },
    }
  }
}
