use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let answer = read_lines("input.txt")
        .map(|result| result.expect("Expect a line."))
        .map(|line| line.chars().collect::<Vec<_>>())
        // map chars to digit values
        .map(|chars| {
            chars
                .into_iter()
                .map(|c| c as u32)
                .map(|d| match d > 96 {
                    true => d - 96,
                    false => d - 64 + 26,
                })
                .collect::<Vec<_>>()
        })
        // put the bags into groups
        .tuples()
        .map(|(bag1, bag2, bag3)| {
            for item in &bag1 {
                if bag2.contains(item) && bag3.contains(item) {
                    return *item;
                }
            }
            panic!("Expected to find a mutual item.");
        })
        .sum::<u32>();

    println!("found answer: {answer}, in {:0.2?}",now.elapsed());
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    match File::open(filename) {
        Ok(file) => io::BufReader::new(file).lines(),
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    }
}
