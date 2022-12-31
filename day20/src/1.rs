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
      let action = if let Some(nr) = right.parse::<u16>().ok() {
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

  let root = monkey_map.get("root").unwrap();

  let answer = get_yell(&root.id, &monkey_map);

  println!("found answer: {} in {:0.2?}", answer, now.elapsed());
}

// can we recurse this deep?
fn get_yell(id: &str, monkey_map: &HashMap<String, Monkey>) -> i64 {
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
