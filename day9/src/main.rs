use std::fs;
use std::str;
use std::time::Instant;

use crate::rope::mv::*;
use crate::rope::*;

pub mod rope;

fn main() {
    let now = Instant::now();
    let mut rope = Rope::new();

    let moves = translations();

    read_file("input.txt")
        .lines()
        .map(|line| line.split(" ").collect())
        .map(|words: Vec<&str>| match words[..] {
            ["R", n] => Move {
                tr: moves.get(&Dir::R).unwrap(),
                n: n.parse::<u8>().unwrap(),
            },
            ["L", n] => Move {
                tr: moves.get(&Dir::L).unwrap(),
                n: n.parse::<u8>().unwrap(),
            },
            ["U", n] => Move {
                tr: moves.get(&Dir::U).unwrap(),
                n: n.parse::<u8>().unwrap(),
            },
            ["D", n] => Move {
                tr: moves.get(&Dir::D).unwrap(),
                n: n.parse::<u8>().unwrap(),
            },
            [] | [_] | [_, ..] => panic!("unexpected match."),
        })
        .for_each(|mv| rope.mv(&mv));

    println!("found answer: {:?}, in {:0.2?}", rope.positions.len(), now.elapsed());
}

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    }
}
