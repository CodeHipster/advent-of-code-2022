use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let answer = read_lines("input.txt")
        .map(|result| result.expect("Expect a line."))
        .map(|stream| {
            let mut iter = stream.as_bytes().windows(4);
            let mut count = 3;
            while let Some(window) = iter.next() {
                count += 1;
                println!("{window:?}");
                if window.iter().unique().collect::<Vec<_>>().len() == 4 {
                    break;
                }
            }
            count
        })
        .for_each(|count| println!("{:?}", count));

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
