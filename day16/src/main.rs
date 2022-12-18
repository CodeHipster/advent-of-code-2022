use core::panic;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs;
use std::str;
use std::time::Instant;

use itertools::Itertools;

struct State {
  marked: HashSet<u16>, //opened or skipped due to 0 flow rate.
  pos: u16,
  time: u8,
  pressure: u32,
}

impl State {
  fn new(pos: u16) -> State {
    State {
      marked: HashSet::new(),
      pos,
      time: 0,
      pressure: 0,
    }
  }
  fn open(&self, flow: u8) -> State {
    let mut marked = self.marked.clone();
    marked.insert(self.pos);
    if flow == 0 {
      // skip actually opening it.
      return State {
        marked,
        pos: self.pos,
        time: self.time,
        pressure: self.pressure,
      };
    }
    let time = self.time + 1;
    let pressure = self.pressure + (30 - time as u32) * (flow as u32);
    return State {
      marked,
      pos: self.pos,
      time,
      pressure,
    };
  }
  //TODO: would be nice to have the compiler be able to guarantee that target is a valid pos.
  fn mv(&self, target: &u16) -> State {
    State {
      marked: self.marked.clone(),
      pos: *target,
      time: self.time + 1,
      pressure: self.pressure,
    }
  }
}

impl Display for State {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let pos = String::from_utf8(vec![(self.pos >> 8) as u8, self.pos as u8]).unwrap();
    write!(f, "pos: {pos}, time: {}, pressure: {}, marked: [", self.time, self.pressure)?;
    for v in &self.marked{
      let v_pos = String::from_utf8(vec![(v >> 8) as u8, *v as u8]).unwrap();
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
      let connections = connections[23..].split(",").map(|t| parse(t.trim())).collect::<Vec<_>>();
      let valve = Valve::new(id, *flow);
      valve_map.insert(id, valve);
      valve_connections.insert(id, connections);
    });

  let start = valve_map.iter().next().unwrap().0;

  let mut states = vec![State::new(*start)];
  let mut states_checked = 0;
  loop {
    let state = states.pop();
    if let None = state {
      break;
    }
    let state = state.unwrap();
    println!("checking state: {state}");
    let mut options = next_states(&state, valve_connections.get(&state.pos).unwrap(), &valve_map);
    states.append(&mut options);
    states_checked += 1;
  }

  let answer = states_checked;

  println!("found answer: {answer} in {:0.2?}", now.elapsed());
}

fn next_states(current: &State, connections: &Vec<u16>, valve_map: &HashMap<u16, Valve>) -> Vec<State> {
  let mut result = vec![];
  if current.time == 30 {
    return result;
  }
  // TODO: don't allow to go back to previous state
  if !current.marked.contains(&current.pos) {
    let mut opened = current.marked.clone();
    opened.insert(current.pos);
    let flow = valve_map.get(&current.pos).unwrap().flow;
    result.push(current.open(flow));
  }
  for connection in connections {
    result.push(current.mv(connection))
  }
  result
}

fn parse(id: &str) -> u16 {
  let bytes = id.as_bytes();
  let left = bytes[0] as u16;
  let right = bytes[0] as u16;
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
