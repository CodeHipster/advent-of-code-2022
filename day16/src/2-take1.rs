use core::panic;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs;
use std::str;
use std::time::Instant;

use itertools::Itertools;

#[derive(Clone)]
struct State {
  marked: HashSet<u16>, //opened or skipped due to 0 flow rate.
  time: u8,
  pressure: u32,
  operators: (Operator, Operator),
}

#[derive(Clone)]
struct Operator {
  name: String,
  position: u16,
  path: HashSet<u16>, // path since last action, so we can detect running in circles.
}

impl Operator {
  fn new(name: String, position: u16, path: HashSet<u16>) -> Operator {
    Operator { name, position, path }
  }
}

impl State {
  fn new(pos: u16) -> State {
    State {
      time: 0,
      pressure: 0,
      marked: HashSet::new(),
      operators: (Operator::new("me".to_string(), pos, HashSet::new()), Operator::new("elephant".to_string(), pos, HashSet::new())),
    }
  }

  fn next(&self) -> State {
    let mut clone = self.clone();
    clone.time += 1;
    clone
  }

  fn open_me(&mut self, flow: u8) {
    self.pressure = self.pressure + (26 - self.time as u32) * (flow as u32);
    let op = &mut self.operators.0;
    self.marked.insert(op.position);
    op.path = HashSet::new();
    op.path.insert(op.position);
  }

  fn open_elephant(&mut self, flow: u8) {
    // TODO: is the time calculation correct?
    self.pressure = self.pressure + (26 - self.time as u32) * (flow as u32);
    let op = &mut self.operators.1;
    self.marked.insert(op.position);
    op.path = HashSet::new();
    op.path.insert(op.position);
  }

  fn mv_operator(op: &mut Operator, target: u16) -> Option<()> {
    if op.path.contains(&target) {
      return None;
    }
    op.path.insert(op.position);
    op.position = target;
    return Some(());
  }
}

impl Display for State {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "time: {}, pressure: {},marked: [", self.time, self.pressure)?;
    for v in &self.marked {
      let v_pos = String::from_utf8(vec![(*v >> 8) as u8, *v as u8]).unwrap();
      write!(f, "{v_pos},")?;
    }
    writeln!(f, "]")?;

    let op = &self.operators.0;
    let pos = String::from_utf8(vec![(op.position >> 8) as u8, op.position as u8]).unwrap();
    write!(f, "   {}: position: {pos}, path: [", op.name)?;
    for v in &op.path {
      let p = String::from_utf8(vec![(*v >> 8) as u8, *v as u8]).unwrap();
      write!(f, "{p},")?;
    }
    writeln!(f, "]")?;

    let op = &self.operators.1;
    let pos = String::from_utf8(vec![(op.position >> 8) as u8, op.position as u8]).unwrap();
    write!(f, "   {}: position: {pos}, path: [", op.name)?;
    for v in &op.path {
      let p = String::from_utf8(vec![(*v >> 8) as u8, *v as u8]).unwrap();
      write!(f, "{p},")?;
    }
    write!(f, "]")
  }
}

struct Valve {
  id: u16,
  flow: u8,
}

impl Valve {
  fn new(id: u16, flow: u8) -> Valve {
    Valve { id, flow }
  }
}

impl Display for Valve {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let id = String::from_utf8(vec![(self.id >> 8) as u8, self.id as u8]).unwrap();
    write!(f, "{}, {}", id, self.flow)
  }
}

fn main() {
  let now = Instant::now();
  let mut valve_map: HashMap<u16, Valve> = HashMap::new();
  let mut valve_connections: HashMap<u16, Vec<u16>> = HashMap::new();

  read_file("input.txt")
    .lines()
    .map(|line| line.split(";").next_tuple::<(_, _)>().unwrap())
    .for_each(|(valve, connections)| {
      let flow = &valve[23..].parse::<u8>().unwrap();
      let id = parse(&valve[6..8]);
      // println!("valve:{valve}, {id:016b}");
      let connections = connections[23..].split(",").map(|t| parse(t.trim())).collect::<Vec<_>>();
      let valve = Valve::new(id, *flow);
      valve_map.insert(id, valve);
      valve_connections.insert(id, connections);
    });

  let start = parse("AA");

  let mut answer = 0;
  let mut states = vec![State::new(start)];
  let mut states_checked = 0;
  loop {
    let state = states.pop();
    if let None = state {
      break;
    }
    let state = state.unwrap();
    if state.pressure > answer {
      answer = state.pressure;
      println!("highest: {state}");
    }
    // println!("checking state: {state}");
    let mut options = next_states(&state, &valve_connections, &valve_map);
    states.append(&mut options);
    states_checked += 1;
  }

  println!("states checked: {states_checked}");
  println!("found answer: {answer} in {:0.2?}", now.elapsed());
}

fn next_states(current: &State, connections: &HashMap<u16, Vec<u16>>, valve_map: &HashMap<u16, Valve>) -> Vec<State> {
  // println!("resolving state: {current}");
  let mut result = vec![];
  let mut my_moves = vec![];
  // TODO: check nr of open valves reached max?
  if current.time == 26 {
    return result;
  }
  // me
  let op = &current.operators.0;
  let flow = valve_map.get(&op.position).unwrap().flow;
  if flow != 0 && !current.marked.contains(&op.position) {
    let mut n = current.next();
    n.open_me(flow);
    my_moves.push(n);
  }
  let cons = connections.get(&op.position).unwrap();
  for connection in cons {
    let mut n = current.next();
    let op = &mut n.operators.0;
    if let Some(_) = State::mv_operator(op, *connection) {
      my_moves.push(n);
    }
  }

  // for each move I do, add moves elephant makes
  for my_mv in my_moves {
    // elephant
    let op = &my_mv.operators.1;
    let flow = valve_map.get(&op.position).unwrap().flow;
    if flow != 0 && !my_mv.marked.contains(&op.position) {
      let mut n = my_mv.clone();
      n.open_elephant(flow);
      result.push(n);
    }
    let cons = connections.get(&op.position).unwrap();
    for connection in cons {
      let mut n = my_mv.clone();
      let op = &mut n.operators.1;
      if let Some(_) = State::mv_operator(op, *connection) {
        result.push(n);
      }
    }
  }

  // println!("previous state:");
  // println!("{current}");
  // println!("next states");
  // for state in &result{
  //   println!("{state}");
  // }
  // println!("");

  result
}

fn parse(id: &str) -> u16 {
  let bytes = id.as_bytes();
  let left = bytes[0] as u16;
  let right = bytes[1] as u16;
  (left << 8) | right
}

fn read_file(filename: &str) -> String {
  match fs::read_to_string(filename) {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    }
  }
}
