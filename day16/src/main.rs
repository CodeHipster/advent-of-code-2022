mod path;

use core::panic;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs;
use std::str;
use std::time::Instant;

use array_tool::vec::Uniq;
use itertools::Itertools;

#[derive(Clone)]
struct State {
  marked: HashSet<u16>,
  pressure: u32,
  operators: (Operator, Operator),
}

#[derive(Clone)]
struct Operator {
  position: u16,
  time: u8,
}

impl Operator {
  fn new(position: u16, time: u8) -> Operator {
    Operator { position, time }
  }
}

impl State {
  fn new(pos: u16) -> State {
    State {
      marked: HashSet::new(),
      pressure: 0,
      operators: (Operator::new(pos, 0), Operator::new(pos, 0)),
    }
  }

  fn next(&self, distance_map: &HashMap<u16, HashMap<u16, usize>>, valves_to_open: &Vec<Valve>) -> Vec<State> {
    let mut result: Vec<State> = vec![];
    let valves_left = valves_to_open.iter().filter(|v| !self.marked.contains(&v.id)).collect::<Vec<_>>();
    if self.operators.0.time == 26 && self.operators.1.time == 26 {
      // both reached the end of the line.
      return result;
    }
    if self.operators.0.time <= self.operators.1.time {
      // move self.
      for valve in valves_left {
        let distance = distance_map.get(&self.operators.0.position).unwrap().get(&valve.id).unwrap();
        let time = self.operators.0.time + *distance as u8 + 1;
        if time > 26 {
          // reach the end of the time.
          let new_state = State {
            marked: self.marked.clone(),
            pressure: self.pressure,
            operators: (Operator::new(valve.id, 26), self.operators.1.clone()),
          };
          result.push(new_state);
          continue;
        }
        let pressure = self.pressure + (26 - time) as u32 * valve.flow as u32;
        let mut marked = self.marked.clone();
        marked.insert(valve.id);
        let new_state = State {
          marked,
          pressure,
          operators: (Operator::new(valve.id, time), self.operators.1.clone()),
        };
        result.push(new_state);
      }
    } else {
      // move the elephant.
      for valve in valves_left {
        let distance = distance_map.get(&self.operators.1.position).unwrap().get(&valve.id).unwrap();
        let time = self.operators.1.time + *distance as u8 + 1;
        if time > 26 {
          let new_state = State {
            marked: self.marked.clone(),
            pressure: self.pressure,
            operators: (self.operators.0.clone(), Operator::new(valve.id, 26)),
          };
          result.push(new_state);
          continue;
        }
        let pressure = self.pressure + (26 - time) as u32 * valve.flow as u32;
        let mut marked = self.marked.clone();
        marked.insert(valve.id);
        let new_state = State {
          marked,
          pressure,
          operators: (self.operators.0.clone(), Operator::new(valve.id, time)),
        };
        result.push(new_state);
      }
    }
    result
  }
}

impl Display for State {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "pressure: {},marked: [", self.pressure)?;
    for v in &self.marked {
      let v_pos = un_parse(v);
      write!(f, "{v_pos},")?;
    }
    writeln!(f, "]")?;

    let op = &self.operators.0;
    write!(f, "   Me: position: {}, time: {}", un_parse(&op.position), op.time)?;

    let op = &self.operators.1;
    write!(f, "   Elephant: position: {}, time: {}", un_parse(&op.position), op.time)
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
    write!(f, "{}, {}", un_parse(&self.id), self.flow)
  }
}

fn main() {
  let now = Instant::now();
  let mut valve_map: HashMap<u16, Valve> = HashMap::new();
  let mut valve_connections: HashMap<u16, Vec<u16>> = HashMap::new();
  let mut valves_to_open: Vec<Valve> = vec![];

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
      if *flow > 0 {
        valves_to_open.push(Valve::new(id, *flow));
      }
    });

  let start = parse("AA");

  let distance_map = distance_map(&valve_connections, start, &valves_to_open);
  print_map(&distance_map);

  let mut states: Vec<State> = vec![State::new(start)];

  let mut answer = 0;
  while let Some(state) = states.pop() {
    if state.pressure > answer {
      answer = state.pressure;
      println!("found state: {state}");
    }
    let mut next_states = state.next(&distance_map, &valves_to_open);
    states.append(&mut next_states);
  }

  println!("found answer: {answer} in {:0.2?}", now.elapsed());
}

fn print_map(map: &HashMap<u16, HashMap<u16, usize>>) {
  for (node, hops) in map {
    let id = String::from_utf8(vec![(node >> 8) as u8, *node as u8]).unwrap();
    println!("Distance from {id}:");
    for (n, h) in hops {
      let i = String::from_utf8(vec![(n >> 8) as u8, *n as u8]).unwrap();
      println!("  {i}:{h}");
    }
    println!("");
  }
}

fn distance_map(valve_connections: &HashMap<u16, Vec<u16>>, start: u16, valves_to_open: &Vec<Valve>) -> HashMap<u16, HashMap<u16, usize>> {
  let mut distance_map: HashMap<u16, HashMap<u16, usize>> = HashMap::new();
  let neighbour_function = |n| valve_connections.get(&n).unwrap().to_vec();
  distance_map.insert(start, path::distance_map(start, neighbour_function));
  for v in valves_to_open {
    distance_map.insert(v.id, path::distance_map(v.id, neighbour_function));
  }
  distance_map
}

fn parse(id: &str) -> u16 {
  let bytes = id.as_bytes();
  let left = bytes[0] as u16;
  let right = bytes[1] as u16;
  (left << 8) | right
}

fn un_parse(id: &u16) -> String {
  String::from_utf8(vec![(id >> 8) as u8, *id as u8]).unwrap()
}

fn read_file(filename: &str) -> String {
  match fs::read_to_string(filename) {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    }
  }
}
