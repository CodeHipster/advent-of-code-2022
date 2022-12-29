use core::time;
use std::{
  collections::{HashSet, HashMap},
  fmt::{Display, Formatter},
  hash::Hash,
};
use lazy_static::lazy_static;

pub struct Blueprint {
  pub id: u8,
  pub ore_bot: u8,
  pub clay_bot: u8,
  pub obsidian_bot: (u8, u8),
  pub geode_bot: (u8, u8),
  pub max_ore_bot: u8,
  pub max_clay_bot: u8,
  pub max_obsidian_bot: u8,
}

impl Display for Blueprint {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Blueprint: {} ", self.id)?;
    writeln!(f, "  Ore bot: {} ore", self.ore_bot)?;
    writeln!(f, "  Clay bot: {} ore", self.clay_bot)?;
    writeln!(f, "  Obsidian bot: {} ore, {} clay", self.obsidian_bot.0, self.obsidian_bot.1)?;
    writeln!(f, "  Geode bot: {} ore, {} obsidian", self.geode_bot.0, self.geode_bot.1)?;

    Ok(())
  }
}

impl Blueprint {
  pub fn new(id: u8, ore_bot: u8, clay_bot: u8, obsidian_bot: (u8, u8), geode_bot: (u8, u8)) -> Blueprint {
    let max_ore_bot = *[ore_bot, clay_bot, obsidian_bot.0, geode_bot.0].iter().max().unwrap();
    let max_clay_bot = obsidian_bot.1;
    let max_obsidian_bot = geode_bot.1;

    Blueprint {
      id,
      ore_bot,
      clay_bot,
      obsidian_bot,
      geode_bot,
      max_ore_bot,
      max_clay_bot,
      max_obsidian_bot,
    }
  }

  // get the quality level of the blueprint
  pub fn quality_level(&self) -> u32 {
    let mut visited: HashSet<State> = HashSet::new();
    let mut todo: Vec<State> = vec![];
    todo.push(State::new());

    let mut geodes: u32 = 0;
    while !todo.is_empty() {
      let current = todo.pop().unwrap();

      if early_exit(&current, geodes){
        continue;
      }

      // if state has more geodes, add them.
      if current.geode as u32 > geodes {
        geodes = current.geode as u32;
        println!("geodes: {geodes}, checked: {}, todo: {}, state: {current}", visited.len(), todo.len())
      }

      // add next states.
      let next = current.next_states(self);
      for s in next {
        if !visited.contains(&s){
          todo.push(s);
        }
      }
      visited.insert(current);
    }

    println!("checked: {}", visited.len());
    geodes as u32
  }
}

fn geodes_per_minute()-> HashMap<u8, u32>{
  let mut result: HashMap<u8, u32> = HashMap::new();
  let mut geodes = 0;
  result.insert(0, geodes);
  for minute in 0..32{
    geodes += minute as u32;
    result.insert(minute +1, geodes);
  }
  result
}

fn early_exit(current: &State, heighest: u32) -> bool {
  // can current state reach more than heighest nr of geodes? -> false

  lazy_static! {
    // geodes per minute
    static ref GPM: HashMap<u8, u32> = geodes_per_minute();
  }

  let time_left = 32-current.time;
  let bots = current.geode_bots as u32;
  let geodes = current.geode as u32;
  let gpm = GPM.get(&time_left).unwrap();

  let max = geodes + bots * time_left as u32 + gpm;

  max <= heighest
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct State {
  time: u8,
  ore: u8,
  clay: u8,
  obsidian: u8,
  geode: u8,
  // nr of bots running
  ore_bots: u8,
  clay_bots: u8,
  obsidian_bots: u8,
  geode_bots: u8,
}

impl Display for State {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "time: {}, ore: {}, clay: {}, obsidian: {}, geode: {}, ore_bots: {}, clay_bots: {}, obsidian_bots: {}, geode_bots: {}",
      self.time, self.ore, self.clay, self.obsidian, self.geode, self.ore_bots, self.clay_bots, self.obsidian_bots, self.geode_bots
    )
  }
}

impl State {
  fn new() -> State {
    State {
      time: 0,
      ore: 0,
      clay: 0,
      obsidian: 0,
      geode: 0,
      ore_bots: 1,
      clay_bots: 0,
      obsidian_bots: 0,
      geode_bots: 0,
    }
  }

  fn step_minute(mut self) -> State {
    self.time += 1;
    self.ore += self.ore_bots;
    self.clay += self.clay_bots;
    self.obsidian += self.obsidian_bots;
    self.geode += self.geode_bots;
    self
  }

  fn buy_ore_bot(&self, blueprint: &Blueprint) -> Option<State> {
    if self.ore >= blueprint.ore_bot && self.ore_bots < blueprint.max_ore_bot {
      let mut next = self.clone().step_minute();
      next.ore -= blueprint.ore_bot;
      next.ore_bots += 1;
      return Some(next);
    } else {
      return None;
    }
  }

  fn buy_clay_bot(&self, blueprint: &Blueprint) -> Option<State> {
    if self.ore >= blueprint.clay_bot && self.clay_bots < blueprint.max_clay_bot {
      let mut next = self.clone().step_minute();
      next.ore -= blueprint.clay_bot;
      next.clay_bots += 1;
      return Some(next);
    } else {
      return None;
    }
  }

  fn buy_obsidian_bot(&self, blueprint: &Blueprint) -> Option<State> {
    if self.obsidian_bots < blueprint.max_obsidian_bot && self.ore >= blueprint.obsidian_bot.0 && self.clay >= blueprint.obsidian_bot.1 {
      let mut next = self.clone().step_minute();
      next.ore -= blueprint.obsidian_bot.0;
      next.clay -= blueprint.obsidian_bot.1;
      next.obsidian_bots += 1;
      return Some(next);
    } else {
      return None;
    }
  }

  fn buy_geode_bot(&self, blueprint: &Blueprint) -> Option<State> {
    if self.ore >= blueprint.geode_bot.0 && self.obsidian >= blueprint.geode_bot.1 {
      let mut next = self.clone().step_minute();
      next.ore -= blueprint.geode_bot.0;
      next.obsidian -= blueprint.geode_bot.1;
      next.geode_bots += 1;
      return Some(next);
    } else {
      return None;
    }
  }

  fn next_states(&self, blueprint: &Blueprint) -> Vec<State> {
    let mut result = vec![];
    // exit clause is time, no more states when time is up.
    if self.time == 32 {
      return result;
    }

    // we only have 1 factory so we can only have 1 extra bot.
    // build an ore bot
    if let Some(state) = self.buy_ore_bot(blueprint) {
      result.push(state);
    }

    // build a clay bot
    if let Some(state) = self.buy_clay_bot(blueprint) {
      result.push(state);
    }

    // build an obsidian bot
    if let Some(state) = self.buy_obsidian_bot(blueprint) {
      result.push(state);
    }

    //build a geode bot
    if let Some(state) = self.buy_geode_bot(blueprint) {
      result.push(state);
    }

    // TODO: we can optimize this.
    if result.len() != 4 {
      // could not create all bots, add a state with just the resources.
      result.push(self.clone().step_minute());
    }

    result
  }
}
