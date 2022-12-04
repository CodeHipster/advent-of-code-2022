use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use range_set::{RangeIntersect, RangeCompare};

fn main() {
    let now = Instant::now();
    let answer = read_lines("input.txt")
        .map(|result| result.expect("Expect a line."))
        // map to 2 strings's
        .map(|line| line.split(",").map(str::to_owned).collect::<Vec<_>>())
        // map to Vec<u32>
        .map(|sss| {
            // section strings
            sss.iter()
                // section string
                .map(|ss| {
                    ss.split("-")
                        .map(|s| s.parse::<u32>().unwrap())
                        .tuples::<(_, _)>()
                        .map(|tup| (tup.0..=tup.1)) // map to a range.
                        .next()
                        .unwrap()
                })
                .tuples::<(_, _)>()
                .map(|sections| range_set::range_compare(&sections.0, &sections.1))
                .map(|comp| match comp{
                  RangeCompare::Intersect(RangeIntersect::OverlapsLeft) => 0,
                  RangeCompare::Intersect(RangeIntersect::OverlapsRight) => 0,
                  RangeCompare::Disjoint(_) => 0,
                  _ => 1,
                })
                // we only have 1 pair to compare.
                .next().unwrap()
        })
        .sum::<u32>();

    println!("found answer: {answer:?}, in {:0.2?}", now.elapsed());
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
