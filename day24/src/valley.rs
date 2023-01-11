use std::{collections::HashSet, fmt::Display};

use crate::{blizzard::Blizzard, twod::XY};

pub struct Valley {
  pub blizzards: Vec<Blizzard>,
  pub start: XY,
  pub end: XY,
  pub walls: (XY, XY), //(left top, right bottom)
}

impl Display for Valley {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for y in self.walls.0.y..=self.walls.1.y {
      for x in self.walls.0.x..=self.walls.1.x {
        if x == self.start.x && y == self.start.y {
          write!(f, ".")?;
        } else if x == self.end.x && y == self.end.y {
          write!(f, ".")?;
        } else if y == self.walls.0.y || y == self.walls.1.y {
          write!(f, "#")?;
        } else if x == self.walls.0.x || x == self.walls.1.x {
          write!(f, "#")?;
        }else{
          let blizzes = self.blizzards.iter().filter(|b| b.pos.x == x && b.pos.y == y).collect::<Vec<_>>();
          match blizzes.len(){
            0 => write!(f, ".")?,
            1 => write!(f, "{}", blizzes[0].dir)?,
            length => write!(f, "{length}")?,
          }
        } 
      }
      writeln!(f, "")?;
    }
    Ok(())
  }
}

impl Valley {
  pub fn new(blizzards: Vec<Blizzard>, walls: (XY, XY)) -> Valley {
    let start = XY::new(1, 0);
    let end = walls.1 + (-1, 0);
    Valley {
      blizzards,
      start,
      end,
      walls,
    }
  }

  pub fn occupied(&self) -> HashSet<XY> {
    self.blizzards.iter().map(|b| b.pos).collect::<HashSet<_>>()
  }

  pub fn time_step(&self) -> Valley {
    // for each blizzard, move the blizzard.
    let blizzards = self.blizzards.iter().map(|b| b.shift(self.walls)).collect::<Vec<_>>();

    Valley {
      blizzards,
      start: self.start,
      end: self.end,
      walls: self.walls,
    }
  }
}
