mod blizzard;
mod path_finder;
mod twod;
mod valley;

use std::fs;
use std::str;
use std::time::Instant;

use crate::blizzard::Blizzard;
use crate::twod::Direction;
use crate::twod::XY;
use crate::valley::Valley;

fn main() {
  let now = Instant::now();

  let mut walls = vec![];
  let mut blizzards = vec![];

  let file = read_file("input.txt");
  file
    .lines()
    .enumerate()
    .flat_map(|(y, line)| line.chars().enumerate().map(|(x, c)| (XY::new_usize(x, y), c)).collect::<Vec<_>>())
    .for_each(|(xy, c)| match c {
      '#' => walls.push(xy),
      '<' => blizzards.push(Blizzard::new(xy, Direction::L)),
      'v' => blizzards.push(Blizzard::new(xy, Direction::D)),
      '^' => blizzards.push(Blizzard::new(xy, Direction::U)),
      '>' => blizzards.push(Blizzard::new(xy, Direction::R)),
      '.' => (), // do nothing
      _ => panic!("unexpected char."),
    });

    let min = min(&walls);
    let max = max(&walls);

    let valley = Valley::new(blizzards, (min, max));
    let answer = path_finder::find_path(valley);

  println!("found answer: {} in {:0.2?}", answer, now.elapsed());
}

fn read_file(filename: &str) -> String {
  match fs::read_to_string(filename) {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    }
  }
}

fn min(vec: &Vec<XY>) -> XY {
  let min_x = vec.iter().map(|e| e.x).min().unwrap();
  let min_y = vec.iter().map(|e| e.y).min().unwrap();
  XY::new(min_x, min_y)
}

fn max(vec: &Vec<XY>) -> XY {
  let max_x = vec.iter().map(|e| e.x).max().unwrap();
  let max_y = vec.iter().map(|e| e.y).max().unwrap();
  XY::new(max_x, max_y)
}
