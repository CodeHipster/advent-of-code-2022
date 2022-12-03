// use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let bags = read_lines("input.txt")
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
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // combine into groups of 3
    // TODO: use crate itertools
    // bags.tuples()
    //     .map(|(elf1, elf2, elf3)| (elf1, elf2, elf3))
    //     .for_each(|group| {
    //         println!("{group:?}");
    //     })

    let mut badges = Vec::new();
    'groups: for group in bags.chunks(3) {
        for d in &group[0] {
            if group[1].contains(d) && group[2].contains(d) {
              // println!("adding {:?}, from group {:?}",*d, group);
                badges.push(*d);
                continue 'groups;
            }
        }
    }

    println!("{}", badges.iter().sum::<u32>());

    // println!("sum: {sum}");
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
