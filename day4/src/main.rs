use itertools::Itertools;
use range_set::RangeCompare;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let answer = read_lines("input.txt")
        // expect
        .map(|result| result.expect("Expect a line."))
        // map to 2 strings's ["1-2", "2-3"]
        .map(|line| line.split(",").map(str::to_owned).collect::<Vec<_>>())
        // map to Vec<u32>
        .map(|sss| {
            // section strings e.g. ["1-2", "2-3"]
            sss.iter()
                // section string e.g "1-2"
                .map(|ss| {
                    ss.split("-") // turns it an iterator of digits represented as strings e.g. ["1","2"]
                        .map(|s| s.parse::<u32>().unwrap()) // parse the string to an unsigned int (we don't have negative values)
                        // reduce the iterator by combining 2 values. e.g.  [1,2] becomes (1,2)
                        // we do this, instead of collecting the vec, so we can keep on chaining the iterable.
                        .tuples::<(_, _)>()
                        .map(|tup| (tup.0..=tup.1)) // map to a range. So we can compare the ranges.
                        .next() // we only get 2 values from the split. So there won't be any values after the next.
                        .unwrap() // next returns an Option with None or Some, and we are only interested in Some
                })
                // we now have a iterable of ranges e.g. [[1..=2],[2..=3]]
                .tuples::<(_, _)>() // again turn ranges into tuples so we can keep chaining.
                .map(|sections| range_set::range_compare(&sections.0, &sections.1)) // compare ranges, returns a type of compare result.
                .map(|comp| match comp {
                    // returns Interset or Disjoint with their subtypes. If it intersects in any way, we return 1.
                    RangeCompare::Intersect(_) => 1,
                    _ => 0,
                })
                .next() // again, we only have 1 pair to compare, so ignore the rest.
                .unwrap()
        })
        .sum::<u32>(); // sum up all the 1's

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
