mod mixer;

use core::panic;
use std::collections::HashSet;
use std::fs;
use std::str;
use std::time::Instant;

use crate::mixer::Mixer;


fn main() {
  let now = Instant::now();

  let nrs = read_file("input.txt").lines().map(|line| line.parse::<i16>().unwrap()).collect::<Vec<_>>();

  let mut mixer = Mixer::new(nrs);
  mixer.mix();

  let answer = mixer.answer();

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
