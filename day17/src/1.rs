mod shape;

use core::panic;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str;
use std::time::Instant;
use std::{cmp, fs};

use grid::{grid, Grid};
use shape::Shape;

#[derive(Debug)]
enum JetDirection {
  LEFT,
  RIGHT,
}

struct Cave {
  jets: Vec<JetDirection>,
  shapes: Vec<Shape>,
  grid: Grid<bool>,
}

impl Cave {
  fn new(jets: Vec<JetDirection>) -> Cave {
    let shapes = vec![Shape::line(), Shape::plus(), Shape::il(), Shape::tower(), Shape::square()];
    let mut grid = grid![];
    for _ in 0..3 {
      grid.insert_row(0, vec![false; 7]);
    }
    Cave { jets, grid, shapes }
  }

  fn simulate(&mut self, rocks: u64) {
    let mut jet_index = 0;
    let jet_size = self.jets.len();
    let mut shape_index = 0;
    let shape_size = self.shapes.len();

    // TODO: more efficiently would be a generator stream?
    let mut shape = &self.shapes[shape_index];
    let mut jet = &self.jets[jet_index];
    for r in 0..rocks {
      if r % 100000000 == 0{
        println!("1/10000th");
      }
      //drop a rock until it settles
      let mut shape_pos = (-1, 2); // (row, col) starting point relative to the grid.
      let mut settled = false;
      while !settled {
        shape_pos = self.jet(jet, shape, shape_pos);
        jet_index = (jet_index + 1) % jet_size;
        jet = &self.jets[jet_index];
        match self.gravity(shape, shape_pos) {
          Some(pos) => shape_pos = pos,
          None => {
            // add rocks to grid
            Cave::settle(&mut self.grid, shape, shape_pos);
            settled = true;
          }
        }
      }
      shape_index = (shape_index + 1) % shape_size;
      shape = &self.shapes[shape_index];
      // println!("{self}");
    }
  }

  fn jet(&self, jet: &JetDirection, shape: &Shape, shape_pos: (i32, i32)) -> (i32, i32) {
    // apply jet
    // println!("push: {jet:?}");
    let translation = match jet {
      JetDirection::LEFT => (shape_pos.0, shape_pos.1 - 1),
      JetDirection::RIGHT => (shape_pos.0, shape_pos.1 + 1),
    };

    if translation.1 < 0 || translation.1 + (shape.grid.cols() as i32 - 1) >= 7 {
      // out of bounds, no moving
      return shape_pos;
    }

    // check if we collide with any grid items.

    let collides = shape.points.iter().map(|p| (p.0 + translation.0, p.1 + translation.1)).any(|p| self.collides(p));

    if collides {
      return shape_pos;
    } else {
      return translation;
    }
  }

  fn gravity(&self, shape: &Shape, shape_pos: (i32, i32)) -> Option<(i32, i32)> {
    let translation = (shape_pos.0 + 1, shape_pos.1); // rows / cols
    if translation.0 >= self.grid.rows() as i32 {
      // we have settled on the bottom.
      return None;
    };

    let collides = shape.points.iter().map(|p| (p.0 + translation.0, p.1 + translation.1)).any(|p| self.collides(p));

    if collides {
      // we have settled on an existing rock.
      return None;
    }
    return Some(translation);
  }

  fn collides(&self, point: (i32, i32)) -> bool {
    if point.0 < 0 {
      // point is not on grid yet.
      return false;
    }
    if point.0 >= self.grid.rows() as i32 {
      // going through the bottom
      return true;
    }
    return self.grid[point.0 as usize][point.1 as usize];
  }

  fn settle(grid: &mut Grid<bool>, shape: &Shape, mut shape_pos: (i32, i32)) {
    // TODO: make a rock type? that contains the position which is translated, instead of having to translate at each step.
    let top = shape_pos.0 - (shape.grid.rows() as i32 - 1);
    let to_add = 3 - top;
    for _ in 0..to_add {
      grid.insert_row(0, vec![false; 7]);
      shape_pos.0 += 1; // move the shape along.
    }

    //TODO: fix shape positions.
    //TODO: point type.

    shape
      .points
      .iter()
      .map(|p| (p.0 + shape_pos.0, p.1 + shape_pos.1))
      .for_each(|p| grid[p.0 as usize][p.1 as usize] = true);
  }
}

impl Display for Cave {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let until = cmp::min(10, self.grid.rows());
    for r in 0..until {
      write!(f, "|")?;
      for b in self.grid.iter_row(r) {
        match b {
          true => write!(f, "#")?,
          false => write!(f, ".")?,
        }
      }
      writeln!(f, "|")?;
    }
    Ok(())
  }
}

fn main() {
  let now = Instant::now();

  let jets = read_file("input.txt")
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| match c {
          '<' => JetDirection::LEFT,
          '>' => JetDirection::RIGHT,
          _ => panic!("unexpected char."),
        })
        .collect::<Vec<_>>()
    })
    .next()
    .unwrap();

  let mut cave = Cave::new(jets);
  cave.simulate(1000000000000);

  let answer = cave.grid.rows() -3;

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
