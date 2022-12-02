use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let win = 6;
    let draw = 3;
    let loss = 0;
    // read file into type
    let moves = read_lines("./input.txt")
        .map(|result| result.unwrap())
        .map(|line| line.split(" ").map(str::to_owned).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // println!("{moves:?}");
    // println!("{}", type_of(&moves));

    let mut total = 0;
    for mv in moves{
      let tup = (mv[1].as_str(),mv[0].as_str());
      let score = match tup{
        ("X", "A") => 1 + draw,
        ("X", "B") => 1 + loss,
        ("X", "C") => 1 + win,
        ("Y", "A") => 2 + win,
        ("Y", "B") => 2 + draw,
        ("Y", "C") => 2 + loss,
        ("Z", "A") => 3 + loss,
        ("Z", "B") => 3 + win,
        ("Z", "C") => 3 + draw,
        _ => panic!("unknown combo {:?}", tup),
      };
      println!("{score}");
      total += score;
    }

    println!("score: {total}");
    // match 3 *3
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
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

// fn type_of<T>(_: T) -> &'static str {
//     std::any::type_name::<T>()
// }
