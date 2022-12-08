use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

#[derive(Debug)]
struct Stack(VecDeque<char>); // elements
struct Move(u32, usize, usize); // amount, from, to
struct Docks(Vec<Stack>); // the stacks

impl Docks {
    fn apply(&mut self, mv: &Move) {
        let stacks = &mut self.0;
        let mut temp: VecDeque<char> = VecDeque::new();
        for _ in 0..mv.0 {
            let crt = stacks[mv.1 - 1].0.pop_front().unwrap();
            temp.push_front(crt);
        }
        for crt in temp{
          stacks[mv.2 - 1].0.push_front(crt);
        }
    }
}

impl Display for Docks {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (index, stack) in self.0.iter().enumerate() {
            writeln!(f, "id:{}, stack:{:?}", index+1, stack)?
        }
        Result::Ok(())
    }
}

impl Move {
    fn new(line: &String) -> Move {
        let words = line.split(" ").map(str::to_owned).collect::<Vec<_>>();
        Move(
            words[1].parse::<u32>().unwrap(),
            words[3].parse::<usize>().unwrap(),
            words[5].parse::<usize>().unwrap(),
        )
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "amount: {}, from:{}, to:{}", self.0, self.1, self.2)
    }
}

fn main() {
    let now = Instant::now();

    let stack1 = Stack(VecDeque::from(['R','W','F','H','T','S',]));
    let stack2 = Stack(VecDeque::from(['W','Q','D','G','S',]));
    let stack3 = Stack(VecDeque::from(['W','T','B',]));
    let stack4 = Stack(VecDeque::from(['J','Z','Q','N','T','W','R','D',]));
    let stack5 = Stack(VecDeque::from(['Z','T','V','L','G','H','B','F',]));
    let stack6 = Stack(VecDeque::from(['G','S','B','V','C','T','P','L',]));
    let stack7 = Stack(VecDeque::from(['P','G','W','T','R','B','Z',]));
    let stack8 = Stack(VecDeque::from(['R','J','C','T','M','G','N',]));
    let stack9 = Stack(VecDeque::from(['W','B','G','L',]));

    let mut docks = Docks(vec![stack1, stack2, stack3,stack4, stack5, stack6,stack7, stack8, stack9]);

    // let stack1 = Stack(VecDeque::from(['N', 'Z']));
    // let stack2 = Stack(VecDeque::from(['D', 'C', 'M']));
    // let stack3 = Stack(VecDeque::from(['P']));

    // let mut docks = Docks(vec![stack1, stack2, stack3]);
    println!("start docks: {docks}");

    read_lines("input.txt")
        .map(|result| result.expect("Expect a line."))
        .filter(|line| line.starts_with("move"))
        .map(|line| Move::new(&line))
        .map(|mv| {
            println!("{}", &mv);
            mv
        })
        .for_each(|mv| {
            docks.apply(&mv);
            println!("{}", &docks)
        });

    let answer: String = docks
        .0
        .iter()
        .map(|stack| stack.0.front().unwrap().to_owned())
        .collect::<String>();

    println!("found answer: {answer:?}, in {:0.2?}", now.elapsed());
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    // open the file
    match File::open(filename) {
        // returns a Result, which is either Ok or Err.
        Ok(file) => io::BufReader::new(file).lines(),
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    }
}
