mod monkey;

use core::panic;
use std::collections::HashMap;
use std::fs;
use std::str;
use std::time::Instant;

use crate::monkey::Action;
use crate::monkey::Monkey;

fn main() {
  let now = Instant::now();

  let monkeys = read_file("input.txt")
    .lines()
    .map(|line| {
      let mut split1 = line.split(":");
      let id = split1.next().unwrap().to_owned();
      let right = split1.next().unwrap().trim();
      let action = if let Some(nr) = right.parse::<i64>().ok() {
        Action::Nr(nr)
      } else {
        let op = right.split(" ").collect::<Vec<_>>();
        match op[..] {
          [left, "+", right] => Action::Add(left.to_owned(), right.to_owned()),
          [left, "-", right] => Action::Sub(left.to_owned(), right.to_owned()),
          [left, "*", right] => Action::Mul(left.to_owned(), right.to_owned()),
          [left, "/", right] => Action::Div(left.to_owned(), right.to_owned()),
          _ => panic!("unexpected value. {op:?}"),
        }
      };
      Monkey { id, action }
    })
    .collect::<Vec<_>>();

  let mut monkey_map = HashMap::new();
  for monkey in monkeys {
    monkey_map.insert(monkey.id.clone(), monkey);
  }

  let path = get_human_path(&monkey_map);

  // left side branch has human.
  let root = monkey_map.get("root").unwrap();
  let start = if let Action::Add(left, right) = &root.action {
    (left.clone(), right.clone())
  } else {
    panic!("ah oh")
  };

  let target = get_yell(&start.1, &monkey_map);
  let answer = calculate_yell(&start.0, &mut monkey_map, target, &path);
  println!("found answer: {} in {:0.2?}", answer, now.elapsed());

  // let answer = 0;
}

fn calculate_yell(id: &str, monkey_map: &HashMap<String, Monkey>, target: i64, human_path: &Vec<String>) -> i64 {
  if id == "humn" {
    return target;
  }
  
  let monkey = &monkey_map.get(id).unwrap();
  println!("calculating yell for: {monkey:?}");
  match &monkey.action {
    Action::Add(left, right) => {
      // first monkey path then human path.
      let ordered = if human_path.contains(right) { (left, right) } else { (right, left) };
      let known = get_yell(ordered.0, monkey_map);
      let target = target - known;
      calculate_yell(ordered.1, monkey_map, target, human_path)
    }
    Action::Sub(left, right) => {
      if human_path.contains(right) {
        let known = get_yell(left, monkey_map);
        let target = known - target;
        calculate_yell(right, monkey_map, target, human_path)
      } else {
        let known = get_yell(right, monkey_map);
        let target = known + target;
        calculate_yell(left, monkey_map, target, human_path)
      }
    }
    Action::Mul(left, right) => {
      // first monkey path then human path.
      let ordered = if human_path.contains(right) { (left, right) } else { (right, left) };
      let known = get_yell(ordered.0, monkey_map);
      let target = target / known;
      calculate_yell(ordered.1, monkey_map, target, human_path)
    }
    Action::Div(left, right) => {
      if human_path.contains(right) {
        let known = get_yell(left, monkey_map);
        let target = known / target;
        calculate_yell(right, monkey_map, target, human_path)
      } else {
        let known = get_yell(right, monkey_map);
        let target = known * target;
        calculate_yell(left, monkey_map, target, human_path)
      }
    }
    _ => panic!("unexpected action: {monkey:?}"),
  }
}

fn get_human_path(monkey_map: &HashMap<String, Monkey>) -> Vec<String> {
  let mut path = vec![];

  let mut parent = &monkey_map.get("humn").unwrap().id;
  while parent != "root" {
    parent = monkey_map
      .iter()
      .find(|(_, m)| match &m.action {
        Action::Add(left, right) | Action::Sub(left, right) | Action::Mul(left, right) | Action::Div(left, right) => left == parent || right == parent,
        _ => false,
      })
      .unwrap()
      .0;
    path.push(parent.clone());
  }

  path
}

fn get_yell(id: &str, monkey_map: &HashMap<String, Monkey>) -> i64 {
  if id == "humn" {panic!("shouldn't get yell from humn")};
  let monkey = monkey_map.get(id).unwrap();
  match &monkey.action {
    Action::Nr(nr) => *nr as i64,
    Action::Add(left, right) => get_yell(&left, monkey_map) + get_yell(&right, monkey_map),
    Action::Sub(left, right) => get_yell(&left, monkey_map) - get_yell(&right, monkey_map),
    Action::Mul(left, right) => get_yell(&left, monkey_map) * get_yell(&right, monkey_map),
    Action::Div(left, right) => get_yell(&left, monkey_map) / get_yell(&right, monkey_map),
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
