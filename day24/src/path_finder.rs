use std::collections::{BTreeSet, HashMap, HashSet};

use crate::{twod::XY, valley::Valley};

// we can derive ordering, it will first compare distance(less is better), then time(less is better),
// then expidition(which makes little sense to compare).
#[derive(Ord, PartialOrd, PartialEq, Eq, Hash)]
struct State {
  distance: i64,
  time: u32,
  expedition: XY,
}

impl State {
  fn initial(valley: &Valley) -> State {
    let expedition = valley.start;
    let diff = valley.end - valley.start;
    let distance = diff.manhattan();
    State { time: 0, expedition, distance }
  }
}

// depth first search, with heuristics
pub fn find_path(valley: Valley) -> u32 {
  // keep map of blizzard state with regards to time.
  // <time, <(blizzard location, blizzards)>>
  let mut blizzard_time_map: HashMap<u32, HashSet<XY>> = HashMap::new();
  let mut valley_time_map: HashMap<u32, Valley> = HashMap::new();
  let mut states: BTreeSet<State> = BTreeSet::new();
  let mut visited: HashSet<State> = HashSet::new();

  // start state:
  let end_xy = valley.end;
  let start = State::initial(&valley);
  blizzard_time_map.insert(start.time, valley.occupied());
  valley_time_map.insert(start.time, valley);

  // TODO: should we keep track of states where we have been?

  states.insert(start);

  let mut shortest_time = u32::MAX;

  while !states.is_empty() {
    let state = states.pop_first().unwrap();

    // do not handle state which will never be able to make it in a faster time.
    if (state.time + state.distance as u32) >= shortest_time {
      continue;
    }

    if visited.contains(&state){
      continue;
    }
    
    if state.distance == 0 {
      // reached target
      if state.time >= shortest_time {
        // we can return because the states are ordered on lowest time first.
        return shortest_time;
      } else {
        shortest_time = state.time;
        println!("Found path in {shortest_time} steps");
        println!("States visited: {}, States to do: {}", visited.len(), states.len());
        continue;
      }
    }

    let new_time = state.time + 1;
    let bliz = blizzard_time_map.get(&new_time);
    if let None = bliz {
      // calculate new state for blizzard at time.
      let current_valley = valley_time_map.get(&state.time).unwrap();
      let new_valley = current_valley.time_step();
      let new_bliz = new_valley.occupied();
      blizzard_time_map.insert(new_time, new_bliz);
      valley_time_map.insert(new_time, new_valley);
    };
    let bliz = blizzard_time_map.get(&new_time).unwrap();
    let valley = valley_time_map.get(&new_time).unwrap();

    // check where the expedition can move.
    //TODO: can't run through walls.
    let mut sides = state.expedition.all_sides();
    sides = remove_walls(&sides, &valley.walls);
    sides.iter().filter(|s| !bliz.contains(s)).for_each(|s| {
      // add state to tree
      let distance = (end_xy - s).manhattan();
      states.insert(State {
        time: new_time,
        expedition: *s,
        distance,
      });
    });
    visited.insert(state);
  }

  // print winning state
  for i in 0..valley_time_map.len(){
    println!("Minute: {i}");
    println!("{}",valley_time_map.get(&(i as u32)).unwrap());
  }

  // ran out of states, return the shortest time to end.
  shortest_time
}

// (min, max)
fn remove_walls(sides: &Vec<XY>, walls: &(XY, XY)) -> Vec<XY> {
  sides
    .iter()
    .filter(|s| {
      if s.x < walls.1.x && s.x > walls.0.x && s.y > walls.0.y && s.y < walls.1.y {
        return true;
      } else if **s == (XY { x: 1, y: 0 }) || **s == (walls.1 + (-1, 0)) {
        return true;
      } else {
        return false;
      }
    })
    .map(|s| s.to_owned())
    .collect::<Vec<_>>()
}
