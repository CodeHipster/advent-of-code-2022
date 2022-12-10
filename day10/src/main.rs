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
}

impl Display for Cpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "signal: {}", self.signal)
    }
}

impl Cpu {
    fn new() -> Cpu {
        Cpu { signal: 1 }
    }

    fn execute(&mut self, program: impl IntoIterator<Item = Instruction>) -> Vec<String> {
        let mut display = vec![];
        let mut line = "".to_owned();
        let mut crt_pos = 0;
        program.into_iter().for_each(|i| {
            // get sprite range
            let spr_range = (self.signal - 1)..=(self.signal + 1);
            // get horizontal crt_position
            // move to new line if over the end.
            if crt_pos > 39 {
                crt_pos = crt_pos % 40;
                display.push(line.to_owned());
                line = "".to_owned();
            }

            // draw on screen
            if spr_range.contains(&crt_pos) {
                line.push('#');
            } else {
                line.push(' ');
            }
            // move crt position
            crt_pos += 1;

            // process instruction
            match i {
                Instruction::Add(val) => self.signal += val,
                Instruction::Noop => (),
            }
        });
        // push the last line
        display.push(line.to_owned());
        display
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
