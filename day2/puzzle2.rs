use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let win = 6;
    let draw = 3;
    let loss = 0;
    let r = 1;
    let p = 2;
    let s = 3;
    // read file into type
    let score = read_lines("./input.txt")
        // unwrap the result to the Ok value.  
        .map(|result| result.unwrap())
        // splitting the line and taking ownership of the value.
        // we need ownership as the line goes out of scope and we want to keep using it.
        //TODO: is this true? to_owned probably allocates new heap space, which is not required as we only read.
        // we need to specify the type to collect it in,
        // but for the the type of the collection we do not need to specify the type
        // we could type Vec<String>
        .map(|line| line.split(" ").map(str::to_owned).collect::<Vec<_>>())
        // map to string references
        .map(|hands| )
        // map the text input resembling the hands that need to be played to a score
        .map(|hands| {
            // map into a tuple containing references, as we can compare these to literals.
            let tup = (hands[0].as_str(), hands[1].as_str());
            // match expression returns a u32
            match tup {
                ("A", "X") => loss + s,
                ("A", "Y") => draw + r,
                ("A", "Z") => win + p,
                ("B", "X") => loss + r,
                ("B", "Y") => draw + p,
                ("B", "Z") => win + s,
                ("C", "X") => loss + p,
                ("C", "Y") => draw + s,
                ("C", "Z") => win + r,
                _ => panic!("unknown combo {:?}", tup),
            }
        })
        .sum::<u32>();

    // println!("{}", type_of(&moves));

    println!("score: {score}");
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
