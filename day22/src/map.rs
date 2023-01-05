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
      if next.variant == TileType::Open {
        self.loc = next.loc;
        self.path.push((self.loc.clone(), self.face.clone()))
      } else {
        // hitting rock, stop walking.
        break;
      }
    }
  }

  fn next_tile(&self, map: &Map) -> Tile {
    // get new location
    let (row, col) = match self.face {
      Direction::Up => (self.loc.row as i32 - 1, self.loc.col as i32),
      Direction::Down => (self.loc.row as i32 + 1, self.loc.col as i32),
      Direction::Left => (self.loc.row as i32, self.loc.col as i32 - 1),
      Direction::Right => (self.loc.row as i32, self.loc.col as i32 + 1),
    };

    // if out of bounds, we wrap.
    if !map.in_bounds(row, col) {
      return self.wrap(map);
    }

    // if stepping into the void, we wrap.
    let tile = map.tiles.get(row as usize, col as usize).unwrap();
    if *tile == TileType::Void{
      return self.wrap(map);
    }

    // else we just step.
    Tile {
      loc: Location { row: row as usize, col: col as usize },
      variant: map.tiles.get(row as usize, col as usize).unwrap().clone(),
    }
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

  fn wrap(&self, map: &Map) -> Tile {
    // return the tile that is on the other side.
    match self.face {
      Direction::Up => {
        let tile = map.tiles.iter_col(self.loc.col).enumerate().rev().filter(|(_, t)| **t != TileType::Void).next().unwrap();
        Tile {
          loc: Location { row: tile.0, col: self.loc.col },
          variant: tile.1.clone(),
        }
      }
      Direction::Down => {
        let tile = map.tiles.iter_col(self.loc.col).enumerate().filter(|(_, t)| **t != TileType::Void).next().unwrap();
        Tile {
          loc: Location { row: tile.0, col: self.loc.col },
          variant: tile.1.clone(),
        }
      }
      Direction::Left => {
        let tile = map.tiles.iter_row(self.loc.row).enumerate().rev().filter(|(_, t)| **t != TileType::Void).next().unwrap();
        Tile {
          loc: Location { row: self.loc.row, col: tile.0 },
          variant: tile.1.clone(),
        }
      }
      Direction::Right => {
        let tile = map.tiles.iter_row(self.loc.row).enumerate().filter(|(_, t)| **t != TileType::Void).next().unwrap();
        Tile {
          loc: Location { row: self.loc.row, col: tile.0 },
          variant: tile.1.clone(),
        }
      }
    }
  }
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
    let first_open_col = self.tiles.iter_row(0).enumerate().filter(|(i, t)| **t == TileType::Open).next().unwrap();
    Location { row: 0, col: first_open_col.0 }
  }

  pub fn in_bounds(&self, row: i32, col: i32) -> bool {
    row >= 0 && (row as usize) < self.tiles.rows() && col >= 0 && (col as usize) < self.tiles.cols()
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
