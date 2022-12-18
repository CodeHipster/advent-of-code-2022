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
  pos: u16,
  previous: u16,
  last_action: u16,
  e_pos: u16,
  e_prev: u16,
  e_last_action: u16,
  time: u8,
  pressure: u32,
}

impl State {
  fn new(pos: u16) -> State {
    // TODO: keep a path since last action, if position is in this path, abort, going in circles.
    State {
      marked: HashSet::new(),
      pos,
      previous: pos,
      last_action: pos,
      e_pos: pos,
      e_prev: pos,
      e_last_action: pos,
      time: 0,
      pressure: 0,
    }
  }

  fn next(&self) -> State {
    let mut clone = self.clone();
    clone.time += 1;
    clone.previous = self.pos;
    clone.e_prev = self.e_pos;
    clone
  }

  fn open_elephant(&mut self, flow: u8) {
    self.marked.insert(self.e_pos);
    self.e_last_action = self.e_pos;
    self.pressure = self.pressure + (26 - self.time as u32) * (flow as u32);
  }

  fn open_me(&mut self, flow: u8) {
    self.marked.insert(self.pos);
    self.last_action = self.pos;
    self.pressure = self.pressure + (26 - self.time as u32) * (flow as u32);
  }

  fn mv_elephant(&mut self, target: u16) {
    self.e_prev = self.e_pos;
    self.e_pos = target;
  }
  fn mv_me(&mut self, target: u16) {
    self.previous = self.pos;
    self.pos = target;
  }
}

impl Display for State {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let pos = String::from_utf8(vec![(self.pos >> 8) as u8, self.pos as u8]).unwrap();
    let e_pos = String::from_utf8(vec![(self.e_pos >> 8) as u8, self.e_pos as u8]).unwrap();
    let previous = String::from_utf8(vec![(self.previous >> 8) as u8, self.previous as u8]).unwrap();
    let e_prev = String::from_utf8(vec![(self.e_prev >> 8) as u8, self.e_prev as u8]).unwrap();
    write!(f, "time: {}, pressure: {}, me: {pos}, me_prev: {previous}, elephant: {e_pos}, e_prev: {e_prev} ,marked: [", 
    self.time, self.pressure)?;
    for v in &self.marked {
      let v_pos = String::from_utf8(vec![(*v >> 8) as u8, *v as u8]).unwrap();
      write!(f, "{v_pos},")?;
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

  read_file("test.txt")
    .lines()
    .map(|line| line.split(";").next_tuple::<(_, _)>().unwrap())
    .for_each(|(valve, connections)| {
      let flow = &valve[23..].parse::<u8>().unwrap();
      let id = parse(&valve[6..8]);
      println!("valve:{valve}, {id:016b}");
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
  if current.time == 26 {
    return result;
  }
  // me
  let flow = valve_map.get(&current.pos).unwrap().flow;
  if flow != 0 && !current.marked.contains(&current.pos) {
    let mut n = current.next();
    n.open(current.pos, flow);
    // println!("added: {n}");
    my_moves.push(n);
  }
  let cons = connections.get(&current.pos).unwrap();
  for connection in cons {
    if *connection != current.previous {
      let mut n = current.next();
      n.mv_me(*connection);
      // println!("added: {n}");
      my_moves.push(n);
    }
  }

  // for each move I do, add moves elephant makes
  for my_mv in my_moves {
    // elephant
    let flow = valve_map.get(&current.e_pos).unwrap().flow;
    if flow != 0 && !my_mv.marked.contains(&current.e_pos) {
      let mut n = my_mv.clone();
      n.open(current.e_pos, flow);
      // println!("added: {n}");
      result.push(n);
    }
    let cons = connections.get(&current.e_pos).unwrap();
    for connection in cons {
      if *connection != current.e_prev {
        let mut n = my_mv.clone();
        n.mv_elephant(*connection);
        // println!("added: {n}");
        result.push(n);
      }
    }
  }
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
