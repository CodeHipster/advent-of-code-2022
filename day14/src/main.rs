use core::panic;
use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::{Range, RangeInclusive};
use std::str;
use std::time::Instant;

use grid::Grid;
use itertools::Itertools;

#[derive(Default, Debug)]
enum Element {
  Source,
  Sand,
  Rock,
  #[default]
  Air,
}

enum MoveResult {
  Air,
  Solid,
  Void,
  None,
}

struct Area {
  grid: Grid<Element>,
  source: Point,
}

impl Area {
  fn new(points: &Vec<Point>) -> Area {
    let max_y = 182;
    let bedrock = max_y + 2;
    let min_x = 500 - bedrock;
    let width = (bedrock * 2) + 1;
    let height = bedrock + 1;
    let mut grid: Grid<Element> = Grid::new(height, width);
    let source = Point { x: 500 - min_x, y: 0 };
    // add other elements.
    // add bedrock.
    for x in 0..width{
      *grid.get_mut(bedrock,x).unwrap() = Element::Source;
    }
    *grid.get_mut(source.y, source.x).unwrap() = Element::Source;
    points.iter().for_each(|p| *grid.get_mut(p.y, p.x - min_x).unwrap() = Element::Rock);
    // create area.
    Area {grid, source }
  }

  // returns false if the sand falls into the void.
  fn add_sand(&mut self) -> bool {
    let mut sand = self.source.clone();
    let mut count = 0;
    loop {
      let result = self.move_sand(&mut sand);
      match result {
        MoveResult::Air => count+=1,
        MoveResult::None => {
          // sand didn't move.
          *self.grid.get_mut(sand.y, sand.x).unwrap() = Element::Sand;
          break;
        } // TODO: add the sand to the grid.
        MoveResult::Void => panic!("shouldn't go into the void."),
        _ => panic!("unexpected move result."),
      };
    }
    if count == 0 {
      // reached the source.
      return false;
    }

    true
  }

  fn move_sand(&mut self, sand: &mut Point) -> MoveResult {
    // check if we can move down
    let options = vec![(0, 1), (-1, 1), (1, 1)]; // down, down left, down right
    for o in options {
      let result = self.try_move(sand, o.0, o.1);
      match result {
        MoveResult::Air => return MoveResult::Air,
        MoveResult::Solid => (), // check other directions.
        MoveResult::Void => return MoveResult::Void,
        _ => panic!("unexpected move result."),
      };
    }
    // all directions are solid, we can't move.
    MoveResult::None
  }

  fn try_move(&self, sand: &mut Point, x_dir: i32, y_dir: i32) -> MoveResult {
    // check if we can move in direction
    // TODO: kinda ugly with all the type casts... but the grid wants usize
    let x = (sand.x as i32) + x_dir;
    let y = (sand.y as i32) + y_dir;
    if let Some(elem) = self.grid.get(y as usize, x as usize) {
      // println!("Found element: {elem:?} at x:{x}, y:{y}");
      if let Element::Air = elem {
        // we can move into given direction.
        sand.x = x as usize;
        sand.y = y as usize;
        return MoveResult::Air;
      }
    } else {
      // went out of bounds.
      return MoveResult::Void;
    };
    MoveResult::Solid
  }
}

impl Display for Area {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    for r in 0..self.grid.rows() {
      for space in self.grid.iter_row(r) {
        match space {
          Element::Source => write!(f, "+")?,
          Element::Sand => write!(f, "o")?,
          Element::Rock => write!(f, "#")?,
          Element::Air => write!(f, " ")?,
        }
      }
      writeln!(f, "")?
    }
    Ok(())
  }
}

enum Direction {
  Horizontal,
  Vertical,
}

#[derive(Clone, Default)]
struct Point {
  x: usize,
  y: usize,
}

impl Point {
  fn new(text: &str) -> Point {
    let cs = text.split(",").collect::<Vec<_>>();
    Point {
      x: cs[0].parse::<usize>().unwrap(),
      y: cs[1].parse::<usize>().unwrap(),
    }
  }
  // create copy with different x
  fn with_x(&self, x: usize) -> Point {
    Point { x, y: self.y }
  }
  // create copy with different y
  fn with_y(&self, y: usize) -> Point {
    Point { x: self.x, y }
  }

  fn between(left: Point, right: Point) -> Vec<Point> {
    let mut result = vec![];
    let range = Point::range(&left, &right); // range includes left and right
    match range.1 {
      Direction::Horizontal => {
        for x in range.0 {
          let p = left.with_x(x);
          result.push(p);
        }
      }
      Direction::Vertical => {
        for y in range.0 {
          let p = left.with_y(y);
          result.push(p);
        }
      }
    }
    result
  }
  fn range(left: &Point, right: &Point) -> (RangeInclusive<usize>, Direction) {
    let dir = if left.x != right.x {
      (left.x, right.x, Direction::Horizontal)
    } else {
      (left.y, right.y, Direction::Vertical)
    };
    if dir.0 < dir.1 {
      (dir.0..=dir.1, dir.2)
    } else {
      (dir.1..=dir.0, dir.2)
    }
  }
}
impl Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{},{}]", self.x, self.y)
  }
}

fn main() {
  let now = Instant::now();

  let points = read_file("input.txt")
    .lines()
    .flat_map(|line| {
      line
        .split(" -> ")
        .map(|c| {
          // map to points in grid
          Point::new(c)
        })
        .tuple_windows::<(_, _)>()
        .flat_map(|(left, right)| Point::between(left, right))
    })
    .collect::<Vec<_>>();

  let mut area = Area::new(&points);
  // println!("{area}");

  let mut answer = 1;
  while let true = area.add_sand(){
    answer += 1;
  }

  // println!("{area}");
  println!("found answer: {answer} in {:0.2?}", now.elapsed());
}

fn read_file(filename: &str) -> String {
  match fs::read_to_string(filename) {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    }
  }
}
