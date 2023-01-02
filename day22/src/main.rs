mod map;

use std::fs;
use std::str;
use std::time::Instant;
use grid::*;
use regex::Regex;

use crate::map::Action;
use crate::map::TileType;
use crate::map::TurnType;

fn main() {
  let now = Instant::now();

  let file = read_file("test.txt");
  let grid_width = file.lines().take_while(|line| *line != "" ).map(|line| line.len()).max().unwrap();
  let mut lines = file.lines();

  let mut grid = grid![[]];
  let mut actions = vec![];
  let regex = Regex::new(r"((?P<digit>\d+)|(?P<letter>[A-Z]))").unwrap();
  while let Some(line) = lines.next() {
    if line == "" {
      let action_line = lines.next().unwrap();
      // parse actions.
      for capture in regex.captures_iter(action_line) {
        let action = if let Some(turn) = capture.name("letter"){
          match turn.as_str() {
            "L" => Action::Turn(TurnType::Left),
            "R" => Action::Turn(TurnType::Right),
            _ => panic!("unexpected turn type.")
          }
        } else if let Some(mv) = capture.name("digit"){
          Action::Move(mv.as_str().parse::<usize>().unwrap())
        }else { panic!("unknown group.")};
        actions.push(action);
      }
    }else{
      // line is a grid line
      let mut row = vec![TileType::Void;grid_width];
      line.chars().map(|c| match c {
        ' ' => TileType::Void,
        '.' => TileType::Open,
        '#' => TileType::Rock,
        _ => panic!("unexpected char."),
      }).enumerate().for_each(|(k,v)|row[k] = v);
      grid.push_row(row);
    }
  }

  

  let answer = 0;

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
