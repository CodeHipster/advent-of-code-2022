use std::fs;
use std::str;
use std::time::Instant;

use itertools::Itertools;
use serde_json::Value;

fn main() {
  let now = Instant::now();

  let file = read_file("test.txt");

  let mut answer = 0;
  let mut index = 1;

  file.lines().tuples().for_each(|(left, right, _)| {
    println!("\n== Pair {index} ==");
    // compare left to right
    let left: Value = serde_json::from_str(left).unwrap();
    let right: Value = serde_json::from_str(right).unwrap();
    match compare(&left, &right, 0) {
      Some(false) => println!(", so inputs are not in the right order"),
      Some(true) => {
        println!(", so inputs are in the right order");
        answer += index;
      }
      None =>{
        println!("- Left side ran out of items, so inputs are in the right order");
        answer += index;
      }
    }
    index += 1;
  });

  println!("found answer: {answer} in {:0.2?}", now.elapsed());
}

fn compare(left: &Value, right: &Value, level: usize) -> Option<bool> {
  print!("{:pad$}", "", pad = level);
  println!("- Compare: {left} vs {right}");
  match (left, right) {
    (Value::Array(l), Value::Array(r)) => {
      // compare each value.
      for i in 0..l.len() {
        let l_val = &l[i];
        if let Some(r_val) = r.get(i) {
          // if right still has values
          match compare(&l_val, r_val, level + 1){
            Some(val) => return Some(val), // order is decided
            None => continue, // order is undecided
          }
        } else {
          // right has run out of values, not in the right order.
          print!("{:pad$}", "", pad = level);
          print!("- Right side ran out of items");
          return Some(false);
        }
      }
      // left has run out of values, continue.
      None
    }
    (Value::Number(l), Value::Array(_)) => {
      //wrap value in array
      print!("{:pad$}", "", pad = level);
      println!("- Mixed types; convert left to [{l}] and retry comparison");
      let wrap = Value::Array(vec![Value::Number(l.clone())]);
      return compare(&wrap, right,level + 1);
    }
    (Value::Array(_), Value::Number(r)) => {
      print!("{:pad$}", "", pad = level);
      println!("- Mixed types; convert right to [{r}] and retry comparison");
      //wrap value in array
      let wrap = Value::Array(vec![Value::Number(r.clone())]);
      return compare(left, &wrap,level + 1);
    }
    (Value::Number(l), Value::Number(r)) => {
      if l.as_u64() < r.as_u64() {
        print!("{:pad$}", "", pad = level);
        print!("- Left side is smaller");
        return Some(true);
      } else if l.as_u64() > r.as_u64() {
        print!("{:pad$}", "", pad = level);
        print!("- Right side is smaller");
        return Some(false);
      } else {
        return None;
      };
    }
    _ => panic!("un expected input."),
  }
}

fn read_file(filename: &str) -> String {
  match fs::read_to_string(filename) {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    }
  }
}
