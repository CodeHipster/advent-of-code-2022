use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fs;
use std::str;
use std::time::Instant;

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(i32),
}

#[derive(Debug)]
struct Cpu {
    signal: i32,
    cycle: u32,
}

impl Display for Cpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "signal: {}, cycle: {}", self.signal, self.cycle)
    }
}

impl Cpu {
    fn new() -> Cpu {
        Cpu { signal: 1, cycle: 0 }
    }

    fn execute(&mut self, program: impl IntoIterator<Item = Instruction>) -> i32 {
        let mut answer = 0;
        let mut peek = 20;
        program.into_iter().for_each(|i| {
            self.cycle += 1;
            if self.cycle == peek {
                peek += 40;
                answer += self.signal * self.cycle as i32;
            }
            // process instruction
            match i {
                Instruction::Add(val) => self.signal += val,
                Instruction::Noop => (),
            }
            // count cycles
            println!("{self}")
        });
        answer
    }
}

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

    println!("found answer: {answer:?}, in {:0.2?}", now.elapsed());
}

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    }
}
