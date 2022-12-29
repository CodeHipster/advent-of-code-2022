mod grid_3d;

use core::panic;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str;
use std::time::Instant;
use std::{cmp, fs};

use grid_3d::{Grid3D, Point3D};

struct Grid {
  dimensions: Point3D,
  drops: Vec<bool>,
}

impl Grid {
  fn new(x: i32, y: i32, z: i32) -> Grid {
    let x = x+1;
    let y = y+1;
    let z = z+1;
    let size = (x*y*z) as usize;
    Grid {
      dimensions: Point3D { x, y, z },
      drops: vec![false; size],
    }
  }

  fn mark_drops(&mut self, drops: &Vec<Point3D>) {
    for drop in drops {
      let index = self.point3d_to_index(*drop);
      self.drops[index] = true;
    }
  }
}

impl Grid3D for Grid {
  fn dimensions(&self) -> Point3D {
    self.dimensions
  }

  fn is_opaque(&self, point: Point3D) -> bool {
    let i = self.point3d_to_index(point);
    return !self.drops[i];
  }
}

fn main() {
  let now = Instant::now();

  let drops = read_file("input.txt")
    .lines()
    .map(|line| {
      let xyz = line.split(",").map(|c| c.parse::<i32>().unwrap()).collect::<Vec<_>>();
      Point3D { x: xyz[0], y: xyz[1], z: xyz[2] }
    })
    .collect::<Vec<_>>();

  let max_x = drops.iter().map(|p| p.x).max().unwrap();
  let max_y = drops.iter().map(|p| p.y).max().unwrap();
  let max_z = drops.iter().map(|p| p.z).max().unwrap();

  let mut grid = Grid::new(max_x, max_y, max_z);

  grid.mark_drops(&drops);

  let mut surface = 0;
  for drop in &drops {
    let neighbours = grid.get_neighbors(*drop).iter().filter(|n| **n != None).count();
    surface += 6 - neighbours;
  }

  let answer = surface;

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
