use core::panic;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::{Range, RangeInclusive};
use std::str;
use std::time::Instant;

use grid::Grid;
use itertools::Itertools;


fn main() {
  let now = Instant::now();

  let mut beacons = vec![];

  let line = 2000000;
  let mut covered: HashSet<i32> = HashSet::new();

  let points = read_file("input.txt")
    .lines()
    .map(|line| line.split(":").next_tuple::<(_,_)>().unwrap())
    .map(|(sensor, beacon)|{
      let sensor_xy: (&str, &str) = sensor[10..].split(",").next_tuple().unwrap();
      let sensor_x = sensor_xy.0[2..].parse::<i32>().unwrap();
      let sensor_y = sensor_xy.1[3..].parse::<i32>().unwrap();
      let beacon_xy: (&str, &str) = beacon[22..].split(",").next_tuple().unwrap();
      let beacon_x = beacon_xy.0[2..].parse::<i32>().unwrap();
      let beacon_y = beacon_xy.1[3..].parse::<i32>().unwrap();
      beacons.push((beacon_x, beacon_y));
      ((sensor_x, sensor_y),(beacon_x, beacon_y))
    }).for_each(|data| {
      println!("sensor x:{}, sensor y:{}, beacon x:{}, beacon y:{}", data.0.0, data.0.1, data.1.0, data.1.1);
      let range = (data.0.0 - data.1.0).abs() + (data.0.1 - data.1.1).abs();
      println!("range: {range}");
      let line_distance = (data.0.1 - line).abs();
      if line_distance > range {return;}
      for x in data.0.0 - (range - line_distance)..=data.0.0 + (range - line_distance){
        covered.insert(x);
      }
  });

  for beacon in beacons {
    if beacon.1 == line{
      covered.remove(&beacon.0);
    }
  }
    
  let mut answer = covered.len();

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
