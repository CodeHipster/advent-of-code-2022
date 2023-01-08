use std::collections::{HashMap, HashSet};

use crate::twod::{Direction, XY};

pub struct Round {
  nr: usize,
  dir: Direction,
}

impl Round {
  pub fn new() -> Round {
    Round { dir: Direction::N, nr: 1 }
  }

  pub fn next(&mut self){
    self.dir = self.dir.next();
    self.nr += 1;
  }

  // returns next round.
  pub fn play(&self, elfs: &HashSet<XY>) -> HashSet<XY> {
    let mut proposed: HashMap<XY, usize> = HashMap::new();
    //<current, new>
    let mut new_map: HashMap<XY, XY> = HashMap::new();

    let mut no_more_moves = true;
    elfs.iter().for_each(|elf| {
      let all_neighbours = Round::neighbours(elfs, elf.all_sides());
      if all_neighbours.is_empty() {
        // no neighbours, not moving
        new_map.insert(*elf, *elf);
        return;
      }
      no_more_moves = false;
      // check each direction if there is no neighbours
      let mut direction = self.dir.clone();
      for _ in 0..4 {
        let sides = elf.sides(&direction);
        let neighbours = sides.iter().filter(|s| all_neighbours.contains(s)).collect::<Vec<_>>();
        if neighbours.is_empty() {
          //propose move in direction.
          let prop = elf.side(&direction);
          *proposed.entry(prop).or_insert(0) += 1;
          new_map.insert(*elf, prop);
          return;
        }
        direction = direction.next();
      }

        // not being able to move.
        new_map.insert(*elf, *elf);
    });

    if no_more_moves {
      println!("no more moves in round: {}", self.nr)
    }

    //modify new_map to not move for new XY values that have size > 1 in the proposed map.
    proposed.iter().filter(|(_, count)| **count > 1).for_each(|(xy_proposed, _)| {
      new_map.iter_mut().filter(|(_, xy_new)| **xy_new == *xy_proposed).for_each(|(xy_current, xy_new)| {
        // Undo the move by setting new the same as current.
        *xy_new = *xy_current;
      });
    });

    let mut new_elfs: HashSet<XY> = HashSet::new();
    new_map.values().for_each(|elf| {
      new_elfs.insert(*elf);
    });

    new_elfs
  }

  fn neighbours(elfs: &HashSet<XY>, neighbours: Vec<XY>) -> Vec<XY> {
    neighbours.iter().filter(|n| elfs.contains(n)).map(|n| *n).collect::<Vec<_>>()
  }
}
