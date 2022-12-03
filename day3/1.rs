use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let sum = read_lines("input.txt")
        .map(|result| result.expect("For a line we expect there to be a line."))
        .map(|line| line.chars().collect::<Vec<_>>())
        // map chars to digit values
        .map(|chars| {
            // println!("chars: {chars:?}");
            chars
                .into_iter()
                .map(|c| c as u32)
                .map(|d| match d > 96 {
                    true => d - 96,
                    false => d - 64 + 26,
                    _ => panic!("Expect only a-Z"),
                })
                .collect::<Vec<_>>()
        })
        // find common digit in left and right.
        .map(|digits| {
            let half = digits.len() / 2;
            let left = &digits[..half];
            let right = &digits[half..];
            println!("left: {left:?}, right:{right:?}");
            for d in left {
                if right.contains(d) {
                    return *d;
                }
            }
            panic!("we should have 1 digit that is equal.")
        })
        .sum::<u32>();

    println!("sum: {sum}");
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

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}
