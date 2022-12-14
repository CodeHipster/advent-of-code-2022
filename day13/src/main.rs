use core::panic;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;
use std::time::Instant;

fn main() {
  let now = Instant::now();

  let mut index2 = 1;
  let mut index6 = 2;

  // reading the entire file is faster, than going line by line.
  read_file("input.txt").lines().filter(|line| !line.is_empty()).for_each(|line| {
    // read_lines("input.txt").map(|line| line.unwrap()).filter(|line| !line.is_empty()).for_each(|line| {
    // println!("Finding first value in line: {line}");
    line
      .as_bytes()
      .windows(2)
      .map(|b| {
        // print!("comparing {} {}, ({})({})", b[0] as char, b[1] as char, b[0], b[1]);
        match b {
          [91, 93] => {
            // [] : no value is smaller than 2 & 6
            index2 += 1;
            index6 += 1;
            // println!(" -> found empty array -> 2:{index2}, 6:{index6}");
            return true;
          }
          [val, 44] | [val, 93] => {
            // x, | x] : compare value with (50)2 & 54()6
            // println!("-> found: {}", val - 48);
            if val < &54 {
              index6 += 1;
              // println!(" -> 6:{index6}");
            } else {
              // println!(" -> bigger than both");
              return true;
            }
            if val < &50 {
              index2 += 1;
              // println!(" -> 2:{index2}");
            }
            return true;
          }[49, 48] => {
            // 10 : bigger than both
            return true;
          }
          _ => {
            // println!(" -> nopes");
            return false;
          }
        }
      })
      .find(|result| *result);
  });
  //answer:  index 2 = 133, index6 = 195
  //mine... index 2 = 160, index6 = 223
  // 27 & 28 diff
  println!("index 2 = {index2}, index6 = {index6}");
  let answer = index2 * index6;
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

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    match File::open(filename) {
        Ok(file) => io::BufReader::new(file).lines(),
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    }
}