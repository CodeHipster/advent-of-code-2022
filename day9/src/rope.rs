use grid::Grid;
use nalgebra::Vector2;
use std::{collections::{HashSet}, fmt::Display};

use crate::rope::mv::*;

pub mod mv;

#[derive(Debug)]
pub struct Rope {
    pub parts: Vec<Knot>,
    pub positions: HashSet<Vector2<i32>>,
}

impl Rope {
    pub fn new() -> Rope {
        Rope {
            parts: vec![Knot::new(0, 0); 10],
            positions: HashSet::new(),
        }
    }

    pub fn mv(&mut self, mv: &Move) {
        // loop n times
        // println!("moving: {mv:?}");
        for _ in 0..mv.n {
            //move rope head
            self.parts[0] = self.parts[0] + mv.tr;
            // move each rope part
            // TODO: there must be a cleaner way? Like move them into a new list?
            for i in 1..self.parts.len() {
                let previous = &self.parts[i - 1];
                let knot = &self.parts[i];
                self.parts[i] = knot + knot.translation_to_other(previous);
            }

            //add tail pos to hashset
            self.positions.insert(self.parts[9].clone());
        }
        // println!("{self}");
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // find min and max positions
        let max_x = self.parts.iter().map(|part| part.x).max().unwrap();
        let max_y = self.parts.iter().map(|part| part.y).max().unwrap();
        let min_x = self.parts.iter().map(|part| part.x).min().unwrap();
        let min_y = self.parts.iter().map(|part| part.y).min().unwrap();

        // make a grid
        let y_offset = -min_y;
        let x_offset = -min_x;
        let height = max_y - min_y;
        let width = max_x - min_x;
        let mut grid = Grid::init((height + 1) as usize, (width + 1) as usize, ".".to_owned());

        // add knots to the grid
        let head = &self.parts[0];
        grid[(head.y + y_offset) as usize][(head.x + x_offset) as usize] = "H".to_owned();
        self.parts.iter().enumerate().skip(1).for_each(|(index, part)| grid[(part.y + y_offset) as usize][(part.x + x_offset) as usize] = (index).to_string());

        // print the grid
        for r in (0..grid.rows()).rev() {
            for point in grid.iter_row(r) {
                write!(f, "{point}")?
            }
            writeln!(f, "")?
        }

        Ok(())
    }
}

pub type Knot = Vector2<i32>;

// trait is required to add functions to a type from another crate.
trait KnotExt {
    fn translation_to_other(&self, other: &Knot) -> Knot;
}

impl KnotExt for Knot {
    fn translation_to_other(&self, other: &Knot) -> Knot {
        // println!("head: {head:?}, tail: {tail:?}");
        //find the diff
        let diff = other - self;
        //move tail according to diff
        // if x|y absolute > 2, we need to move
        if diff.x == 2 {
            // move right
            if diff.y == 0 {
                // right
                return Knot::new(1, 0);
            } else if diff.y > 0 {
                // right up
                return Knot::new(1, 1);
            } else if diff.y < 0 {
                // right down
                return Knot::new(1, -1);
            }
        } else if diff.x == -2 {
            // move left
            if diff.y == 0 {
                // right
                return Knot::new(-1, 0);
            } else if diff.y > 0 {
                // right up
                return Knot::new(-1, 1);
            } else if diff.y < 0 {
                // right down
                return Knot::new(-1, -1);
            }
        } else if diff.y == 2 {
            // move up
            if diff.x == 0 {
                // up
                return Knot::new(0, 1);
            } else if diff.x > 0 {
                // up right
                return Knot::new(1, 1);
            } else if diff.x < 0 {
                // up left
                return Knot::new(-1, 1);
            }
        } else if diff.y == -2 {
            // move down
            if diff.x == 0 {
                // down
                return Knot::new(0, -1);
            } else if diff.x > 0 {
                // down right
                return Knot::new(1, -1);
            } else if diff.x < 0 {
                // down left
                return Knot::new(-1, -1);
            }
        }
        // no movement required.
        return Knot::new(0, 0);
    }
}
