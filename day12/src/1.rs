use std::fmt::Display;
use std::fmt::Formatter;
use std::fs;
use std::str;
use std::time::Instant;

use grid::Grid;

struct Path {
  height: usize,
  steps: Option<usize>,
}
impl Path {
  fn new(height: usize) -> Path {
    Path { height, steps: None }
  }
}

impl Default for Path {
  fn default() -> Self {
    Path { height: 0, steps: None }
  }
}

struct Area {
  grid: Grid<Path>,
}

#[derive(Copy, Clone)]
struct Point {
  row: usize,
  col: usize,
}

impl Point {
  fn new(row: usize, col: usize) -> Point {
    Point { row, col }
  }
}
impl Display for Point {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "r[{}] c[{}] ", self.row, self.col)
  }
}

impl Display for Area {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    for r in 0..self.grid.rows() {
      for point in self.grid.iter_row(r) {
        if let Some(steps) = point.steps {
          write!(f, "({steps:^4})")?
        } else {
          write!(f, ">{:^4}<", point.height)?
        }
      }
      writeln!(f, "")?
    }
    Ok(())
  }
}

struct PathPoint {
  steps: usize,
  point: Point,
  height: usize,
}

impl PathPoint {
  fn new(steps: usize, point: Point, height: usize) -> PathPoint {
    PathPoint { steps, point, height }
  }
}

impl Display for PathPoint {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "steps: {}, height:{}, point:{}", self.steps, self.height, self.point)
  }
}

fn main() {
  let now = Instant::now();

  let mut area = Area { grid: Grid::new(0, 0) };

  let mut start: Point = Point::new(0, 0);
  let mut end: Point = Point::new(0, 0);

  let file = read_file("input.txt");
  file
    .lines()
    .enumerate()
    .map(|(y, line)| {
      line
        .chars()
        .enumerate()
        .map(|(x, c)| match c {
          'S' => {
            start = Point::new(y as usize, x as usize);
            Path::new(0)
          }
          'E' => {
            end = Point::new(y as usize, x as usize);
            Path::new(25)
          }
          val => Path::new((val as usize) - 97),
        })
        .collect::<Vec<Path>>()
    })
    .for_each(|line| area.grid.push_row(line));

  println!("{area}");

  let mut cont = true;
  let mut least = area.grid.cols() * area.grid.rows();
  // steps, Point, height
  let start_height = area.grid.get(start.row, start.col).unwrap().height;
  let start_steps = 0;
  let start_pos = start.clone();
  let start_path = PathPoint::new(start_steps, start_pos, start_height);
  let mut options = vec![start_path]; // options to move.
  while cont {
    let path_point = options.pop().unwrap();
    // println!("\n----\nchecking path: {path_point}");
    if path_point.point.row == end.row && path_point.point.col == end.col {
      // reached end.
      println!("reached end.");
      if path_point.steps < least {
        least = path_point.steps
      };
      continue; // maybe there is shorter paths.
    }
    // check sides
    // up
    if let Some(val) = check_side(&area.grid, (-1, 0), &path_point) {
      options.push(val);
    }
    // down
    if let Some(val) = check_side(&area.grid, (1, 0), &path_point) {
      options.push(val);
    }
    // left
    if let Some(val) = check_side(&area.grid, (0, -1), &path_point) {
      options.push(val);
    }
    // right
    if let Some(val) = check_side(&area.grid, (0, 1), &path_point) {
      options.push(val);
    }

    // set steps to reach point on grid.
    area.grid.get_mut(path_point.point.row, path_point.point.col).unwrap().steps = Some(path_point.steps);
    cont = !options.is_empty();
    // println!("{area}");
    // println!("options in the list: ");
    // options.iter().for_each(|option| println!("{option}"));
  }
  println!("{area}");

  println!("found answer: {least} in {:0.2?}", now.elapsed());
}
//side (row, col) in 1, 0 , -1
fn check_side(grid: &Grid<Path>, side: (i8, i8), current: &PathPoint) -> Option<PathPoint> {
  // check bounds
  let row = current.point.row as i8 - side.0;
  if row < 0 {
    return None;
  }

  let col = current.point.col as i8 - side.1;
  if col < 0 {
    return None;
  }

  let path_point = grid.get(row as usize, col as usize);
  // None if out of upper bounds.
  if let Some(target) = path_point {
    if (current.height + 1) >= target.height {
      // we can make this step.
      if let Some(target_steps) = target.steps {
        // if this path has already been explored
        if (current.steps + 1) < target_steps {
          //this path is faster.
          return Some(PathPoint::new(current.steps + 1, Point::new(row as usize, col as usize), target.height));
        } else {
          return None;
        }
      }else{
        // not been explored, add it as an option
        return Some(PathPoint::new(current.steps + 1, Point::new(row as usize, col as usize), target.height));
      }
    }
  }
  None
}

fn read_file(filename: &str) -> String {
  match fs::read_to_string(filename) {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    }
  }
}
