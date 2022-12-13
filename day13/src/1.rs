use core::panic;
use std::collections::VecDeque;
use std::fs;
use std::str;
use std::time::Instant;

use itertools::Itertools;
use serde_json::Value;

fn main() {
  let now = Instant::now();

  let file = read_file("input.txt");

  let mut answer = 0;
  let mut index = 1;

  file.lines().tuples().for_each(|(left, right, _)| {
    println!("\n== Pair {index} ==");
    // compare left to right
    let left: Value = serde_json::from_str(left).unwrap();
    let right: Value = serde_json::from_str(right).unwrap();
    match compare(Some(left), Some(right), 0) {
      Some(false) => println!(", so inputs are not in the right order"),
      Some(true) => {
        println!(", so inputs are in the right order");
        answer += index;
      }
      None => {
        panic!("unexpected outcome!");
      }
    }
    index += 1;
  });

  println!("found answer: {answer} in {:0.2?}", now.elapsed());
}

fn compare(left: Option<Value>, right: Option<Value>, level: usize) -> Option<bool> {
  print!("{:pad$}", "", pad = level);
  println!("- Compare: {left:?} vs {right:?}");
  match (left, right) {
    (Some(Value::Array(l)), Some(Value::Array(r))) => {
      // we need to pop_front
      let mut l = VecDeque::from(l);
      let mut r = VecDeque::from(r);
      // compare each value.
      loop {
        let l_val = l.pop_front();
        let r_val = r.pop_front();
        if l_val == None && r_val == None {
          // both lists are empty at the same time, continue i guess?
          return None;
        }
        if let Some(val) = compare(l_val, r_val, level + 1) {
          return Some(val); // order is decided
        }
        // else continue to loop
      }
    }
    (Some(Value::Number(l)), Some(Value::Array(r))) => {
      //wrap value in array
      print!("{:pad$}", "", pad = level);
      println!("- Mixed types; convert left to [{l}] and retry comparison");
      let wrap = Value::Array(vec![Value::Number(l.clone())]);
      return compare(Some(wrap), Some(Value::Array(r)), level + 1);
    }
    (Some(Value::Array(l)), Some(Value::Number(r))) => {
      print!("{:pad$}", "", pad = level);
      println!("- Mixed types; convert right to [{r}] and retry comparison");
      //wrap value in array
      let wrap = Value::Array(vec![Value::Number(r.clone())]);
      return compare(Some(Value::Array(l)), Some(wrap), level + 1);
    }
    (Some(Value::Number(l)), Some(Value::Number(r))) => {
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
    (None, Some(_)) => {
      print!("{:pad$}", "", pad = level);
      print!("- Left side ran out of items");
      return Some(true);
    }
    (Some(_), None) => {
      print!("{:pad$}", "", pad = level);
      print!("- Right side ran out of items");
      return Some(false);
    }
    (None, None) =>{
      // both ran out of items at the same time
      return None;
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
