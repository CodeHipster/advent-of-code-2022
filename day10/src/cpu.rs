use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Add(i32),
}

#[derive(Debug)]
pub struct Cpu {
    signal: i32,
}

impl Display for Cpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "signal: {}", self.signal)
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { signal: 1 }
    }

    pub fn execute(&mut self, program: impl IntoIterator<Item = Instruction>) -> Vec<String> {
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
                let mut swap = "".to_owned();
                // take ownership of the line and replace it with an empty one.
                std::mem::swap(&mut swap, &mut line);
                display.push(swap);
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