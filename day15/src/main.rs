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

  let mut sensors = vec![];

  let line = 2000000;
  let mut covered: HashSet<i32> = HashSet::new();

  let points = read_file("input.txt")
    .lines()
    .map(|line| line.split(":").next_tuple::<(_, _)>().unwrap())
    .map(|(sensor, beacon)| {
      let sensor_xy: (&str, &str) = sensor[10..].split(",").next_tuple().unwrap();
      let sensor_x = sensor_xy.0[2..].parse::<i32>().unwrap();
      let sensor_y = sensor_xy.1[3..].parse::<i32>().unwrap();
      let beacon_xy: (&str, &str) = beacon[22..].split(",").next_tuple().unwrap();
      let beacon_x = beacon_xy.0[2..].parse::<i32>().unwrap();
      let beacon_y = beacon_xy.1[3..].parse::<i32>().unwrap();
      ((sensor_x, sensor_y), (beacon_x, beacon_y))
    })
    .for_each(|data| {
      println!("sensor x:{}, sensor y:{}, beacon x:{}, beacon y:{}", data.0 .0, data.0 .1, data.1 .0, data.1 .1);
      let range = (data.0 .0 - data.1 .0).abs() + (data.0 .1 - data.1 .1).abs();
      sensors.push((data.0 .0, data.0 .1, range));
      println!("range: {range}");
    });

  let mut answer: u64 = 0;
  // check all sensor extremities.
  for sensor in &sensors {
    let edges = get_edges(&sensor);
    for edge in edges {
      if !is_covered(edge.0, edge.1, &sensors) {
        answer = edge.0 as u64 * 4000000 + edge.1 as u64;
      }
    }
  }

  println!("found answer: {answer} in {:0.2?}", now.elapsed());
}

fn get_edges(sensor: &(i32, i32, i32)) -> Vec<(i32, i32)> {
  let max = 4000000;
  // let max = 20;
  let mut edges = vec![];
  let range = sensor.2 + 1;
  let (x, y, _) = sensor;
  for n in 0..range {
    edges.push((x + n, y + (range - n))); // right top
    edges.push((x - n, y - (range - n))); // left bottom
    edges.push((x + (range - n), y + n)); // right bottom
    edges.push((x - (range - n), y - n)); // left top
  }
  // remove the big ones.
  edges.retain(|(x, y)| !((x < &0 || x > &max) || (y < &0 || y > &max)));
  edges
}

fn is_covered(x: i32, y: i32, sensors: &Vec<(i32, i32, i32)>) -> bool {
  for sensor in sensors {
    let dif = (x - sensor.0).abs() + (y - sensor.1).abs();
    if dif <= sensor.2 {
      return true;
    }
  }
  return false;
}

fn read_file(filename: &str) -> String {
  match fs::read_to_string(filename) {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    }
  }
}
