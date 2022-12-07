use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;
use std::time::Instant;
fn main() {
    let now = Instant::now();

    let answer = read_lines("input.txt")
        .map(|result| result.expect("Expect a line."))
        .map(|line| {
            let packet_marker = unique_marker(&line, 4);
            let data = &line[packet_marker..];
            packet_marker + unique_marker(data, 14)
        })
        .for_each(|count| println!("{:?}", count));

    println!("found answer: {answer:?}, in {:0.2?}", now.elapsed());
}

// Return position in string after a unique sequence of length window.
fn unique_marker(line: &str, window: usize) -> usize {
    let mut iter = line.as_bytes().windows(window);
    let mut count = window - 1;
    while let Some(view) = iter.next() {
        count += 1;
        if view.iter().unique().collect::<Vec<_>>().len() == window {
            println!("{:?}", str::from_utf8(view).unwrap());
            break;
        }
    }
    count
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
