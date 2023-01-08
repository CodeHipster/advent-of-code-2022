mod round;
mod twod;

use std::collections::HashSet;
use std::fs;
use std::str;
use std::time::Instant;

use crate::round::Round;
use crate::twod::XY;

fn main() {
  let now = Instant::now();

  let file = read_file("input.txt");
  let mut elfs = file
    .lines()
    .enumerate()
    .flat_map(|(y, line)| line.chars().enumerate().filter(|(_, char)| *char == '#').map(|(x, _)| XY::new_usize(x, y)).collect::<Vec<_>>())
    .collect::<HashSet<_>>();

  print(&elfs);

  let mut round = Round::new();
  // play 10 round on elves.
  for _ in 0..990{
    elfs = round.play(&elfs);
    // print(&elfs);
    round.next();
  };

  let answer = answer(&elfs);

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

fn answer(elfs: &HashSet<XY>) -> i64{
  let min = min(elfs);
  let max = max(elfs);
  let diff = (max - min) + (1,1);

  (diff.x * diff.y) - elfs.len() as i64
}

fn min(elfs: &HashSet<XY>) -> XY {
  let min_x = elfs.iter().map(|e| e.x).min().unwrap();
  let min_y = elfs.iter().map(|e| e.y).min().unwrap();
  XY::new(min_x, min_y)
}
fn max(elfs: &HashSet<XY>) -> XY {
  let max_x = elfs.iter().map(|e| e.x).max().unwrap();
  let max_y = elfs.iter().map(|e| e.y).max().unwrap();
  XY::new(max_x, max_y)
}

fn print(elfs: &HashSet<XY>) {
  let min = min(elfs);
  let max = max(elfs);

  for y in min.y..=max.y {
    for x in min.x..=max.x {
      if elfs.contains(&XY::new(x, y)) {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!("");
  }
  println!();
}
