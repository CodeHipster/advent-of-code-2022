mod blueprint;

use core::panic;
use blueprint::Blueprint;
use lazy_static::lazy_static;
use std::fs;
use std::str;
use std::time::Instant;

use regex::Regex;

fn main() {
  let now = Instant::now();

  let blue_prints = read_file("input.txt").lines().map(|line| parse(line)).collect::<Vec<_>>();

  let mut ql = 0;
  for bp in blue_prints {
    println!("blueprint: {bp}");
    let q = bp.quality_level();
    println!("quality_level: {q}");
    ql += q;
  }

  let answer = ql;

  println!("found answer: {} in {:0.2?}", answer, now.elapsed());
}

fn parse(line: &str) -> Blueprint {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
  }
  let caps = RE.captures_iter(line).map(|c| c.get(1).unwrap()).map(|nr| nr.as_str().parse::<u8>().unwrap()).collect::<Vec<_>>();

  Blueprint::new(
    caps[0],
    caps[1],
    caps[2],
    (caps[3], caps[4]),
    (caps[5], caps[6]),
  )
}

fn read_file(filename: &str) -> String {
  match fs::read_to_string(filename) {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    }
  }
}
