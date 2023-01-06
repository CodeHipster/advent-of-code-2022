use grid::Grid;

#[derive(Clone, PartialEq)]
pub enum TileType {
  Void,
  Rock,
  Open,
}

#[derive(Clone, PartialEq)]
pub struct Tile {
  loc: Location,
  variant: TileType,
}

#[derive(Clone, PartialEq)]
pub struct Location {
  pub row: usize,
  pub col: usize,
}

#[derive(Clone, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Clone, PartialEq)]
pub enum TurnType {
  Left,
  Right,
}

#[derive(Clone, PartialEq)]
pub enum Action {
  Move(usize),
  Turn(TurnType),
}

pub struct Pawn {
  loc: Location,
  face: Direction,
  actions: Vec<Action>,
  pub path: Vec<(Location, Direction)>,
}

impl Pawn {
  pub fn new(loc: Location, face: Direction, actions: Vec<Action>) -> Pawn {
    Pawn { loc, face, actions, path: vec![] }
  }

  pub fn walk(&mut self, map: &Map) {
    // apply actions to self.
    for i in 0..self.actions.len() {
      let action = self.actions[i].clone();
      match action {
        Action::Move(steps) => self.walk_steps(steps, map),
        Action::Turn(direction) => self.turn(direction.clone()),
      }
    }
  }

  pub fn answer(&self) -> usize {
    let mut answer = (self.loc.row + 1) * 1000;
    answer += (self.loc.col + 1) * 4;
    answer += match self.face {
      Direction::Up => 3,
      Direction::Down => 1,
      Direction::Left => 2,
      Direction::Right => 0,
    };
    answer
  }

  fn walk_steps(&mut self, steps: usize, map: &Map) {
    for _ in 0..steps {
      let next = self.next_tile(map);
      let tile_type = map.tiles.get(next.0.row, next.0.col).unwrap();
      if *tile_type == TileType::Open {
        self.loc = next.0;
        self.face = next.1;
        self.path.push((self.loc.clone(), self.face.clone()))
      } else {
        // hitting rock, stop walking.
        break;
      }
    }
  }

  fn next_tile(&self, map: &Map) -> (Location, Direction) {
    // get new location
    let (row, col) = match self.face {
      Direction::Up => (self.loc.row as i32 - 1, self.loc.col as i32),
      Direction::Down => (self.loc.row as i32 + 1, self.loc.col as i32),
      Direction::Left => (self.loc.row as i32, self.loc.col as i32 - 1),
      Direction::Right => (self.loc.row as i32, self.loc.col as i32 + 1),
    };

    // if out of bounds, we wrap.
    if !map.in_bounds(row, col) {
      return self.wrap();
    }

    // if stepping into the void, we wrap.
    let tile = map.tiles.get(row as usize, col as usize).unwrap();
    if *tile == TileType::Void {
      return self.wrap();
    }

    // else we just step.
    (Location { row: row as usize, col: col as usize }, self.face.clone())
  }

  fn turn(&mut self, dir: TurnType) {
    match dir {
      TurnType::Left => self.turn_left(),
      TurnType::Right => self.turn_right(),
    }
  }

  fn turn_left(&mut self) {
    self.face = match self.face {
      Direction::Up => Direction::Left,
      Direction::Down => Direction::Right,
      Direction::Left => Direction::Down,
      Direction::Right => Direction::Up,
    }
  }

  fn turn_right(&mut self) {
    self.face = match self.face {
      Direction::Up => Direction::Right,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
      Direction::Right => Direction::Down,
    }
  }

  fn wrap(&self) -> (Location, Direction) {
    let current_side = Map::get_side(self.loc.row, self.loc.col);

    match current_side {
      Side::Top => {
        // can wrap to back and left.
        match self.face {
          Direction::Up => {
            // move to left side of back
            return (Location { row: self.loc.col + 100, col: 0 }, Direction::Right);
          }
          Direction::Left => {
            // move to left side of left
            return (
              Location {
                row: (49 - self.loc.row) + 100,
                col: 0,
              },
              Direction::Right,
            );
          }
          _ => panic!("un expected move from top side."),
        }
      }
      Side::Left => {
        // can wrap to top and front
        match self.face {
          Direction::Up => {
            // move to left side of front
            return (Location { row: self.loc.col + 50, col: 50 }, Direction::Right);
          },
          Direction::Left => {
            // move to left side of top (upside down)
            return (
              Location {
                row: 49 - (self.loc.row - 100),
                col: 50,
              },
              Direction::Right,
            );
          },
          _ => panic!("un expected move from left side."),
        }
      }
      Side::Right => {
        // can wrap to back, bottom, front
        match self.face {
          Direction::Up => {
            // move to bottom side of back
            return (Location { row: 199, col: self.loc.col - 100 }, Direction::Up);
          },
          Direction::Down => {
            // move to right side of front
            return (Location { row: self.loc.col - 50, col: 99 }, Direction::Left);
          },
          Direction::Right => {
            // move to right side of bottom (upside down)
            return (
              Location {
                row: (49 - self.loc.row) + 100,
                col: 99,
              },
              Direction::Left,
            );
          }
          _ => panic!("un expected move from right side."),
        }
      }
      Side::Front => {
        // can wrap to back, bottom, front
        match self.face {
          Direction::Right => {
            // move to bottom of right
            return (Location { row: 49, col: self.loc.row + 50 }, Direction::Up);
          },
          Direction::Left => {
            // move to top side of left
            return (Location { row: 100, col: self.loc.row - 50 }, Direction::Down);
          },
          _ => panic!("un expected move from front side."),
        }
      }
      //todo: check directions.
      Side::Back => {
        match self.face {
          Direction::Right => {
            // move to bottom of bottom
            return (Location { row: 149, col: self.loc.row - 100 }, Direction::Up);
          },
          Direction::Left => {
            // move to top of top
            return (Location { row: 0, col: self.loc.row - 100 }, Direction::Down);
          },
          Direction::Down => {
            // move to top of right
            return (Location { row: 0, col: self.loc.col + 100 }, Direction::Down);
          },
          _ => panic!("un expected move from back side."),
        }        
      },
      Side::Bottom => {
        match self.face {
          Direction::Right => {
            // move to right side of right (upside down)
            return (
              Location {
                row: (49 - (self.loc.row % 50)),
                col: 149,
              },
              Direction::Left,
            );
          },
          Direction::Down => {
            // move to right side of back.
            return (Location { row: self.loc.col + 100, col: 49 }, Direction::Left);
          },
          _ => panic!("un expected move from back side."),
        }  
      },
    }
  }
}

pub enum Side {
  Top,
  Left,
  Right,
  Front,
  Back,
  Bottom,
}

pub struct Map {
  pub tiles: Grid<TileType>,
}

impl Map {
  pub fn new(tiles: Grid<TileType>) -> Map {
    Map { tiles }
  }

  //left most open tile on the first row.
  pub fn start_pos(&self) -> Location {
    let first_open_col = self.tiles.iter_row(0).enumerate().filter(|(_i, t)| **t == TileType::Open).next().unwrap();
    Location { row: 0, col: first_open_col.0 }
  }

  pub fn in_bounds(&self, row: i32, col: i32) -> bool {
    row >= 0 && (row as usize) < self.tiles.rows() && col >= 0 && (col as usize) < self.tiles.cols()
  }

  pub fn get_side(row: usize, col: usize) -> Side {
    let side_size = 50;
    let between = |value, low, high| value >= low && value < high;

    if between(row, 0 * side_size, 1 * side_size) && between(col, 1 * side_size, 2 * side_size) {
      return Side::Top;
    }
    if between(row, 0 * side_size, 1 * side_size) && between(col, 2 * side_size, 3 * side_size) {
      return Side::Right;
    }
    if between(row, 1 * side_size, 2 * side_size) && between(col, 1 * side_size, 2 * side_size) {
      return Side::Front;
    }
    if between(row, 2 * side_size, 3 * side_size) && between(col, 1 * side_size, 2 * side_size) {
      return Side::Bottom;
    }
    if between(row, 2 * side_size, 3 * side_size) && between(col, 0 * side_size, 1 * side_size) {
      return Side::Left;
    }
    if between(row, 3 * side_size, 4 * side_size) && between(col, 0 * side_size, 1 * side_size) {
      return Side::Back;
    }
    panic!("Unexpected range.")
  }

  pub fn print_path(&self, path: &Vec<(Location, Direction)>) {
    for row in 0..self.tiles.rows() {
      for col in 0..self.tiles.cols() {
        let walked = path.iter().find(|(pos, _)| pos.row == row && pos.col == col);
        let c = if let Some((_, dir)) = walked {
          match dir {
            Direction::Up => "^",
            Direction::Down => "v",
            Direction::Left => "<",
            Direction::Right => ">",
          }
        } else {
          // not walked, print tile
          let tile = self.tiles.get(row, col).unwrap();
          match tile {
            TileType::Void => " ",
            TileType::Rock => "#",
            TileType::Open => ".",
          }
        };
        print!("{c}");
      }
      println!("");
    }
  }
}
