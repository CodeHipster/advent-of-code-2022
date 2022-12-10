use std::fs;
use std::str;
use std::time::Instant;

use crate::cpu::*;

pub mod cpu;

fn main() {
    let now = Instant::now();

    let mut cpu = Cpu::new();

    let file = read_file("input.txt");
    let program = file.lines().map(|line| line.split(" ").collect::<Vec<_>>()).flat_map(|words| match words[..] {
        // flat_map to convert to single cycle instructions.
        ["noop"] => vec![Instruction::Noop],
        ["addx", nr] => vec![Instruction::Noop, Instruction::Add(nr.parse::<i32>().unwrap())],
        [] | [_] | [_, ..] => panic!("unexpected match."),
    });

    let answer = cpu.execute(program);

    for line in answer{
      println!("{line}");
    }

    println!("found answer: in {:0.2?}", now.elapsed());
}

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    }
}
