use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

trait BagExt{
  fn total_calories(&self) -> u32;
}

type Bag = Vec<u32>;

impl BagExt for Bag{
  fn total_calories(&self) -> u32 {
    let mut total = 0;
    for calo in self{
      total += calo;
    }
    total
  }
}

fn main() {
  // read file into type
    let mut elves: Vec<Bag> = Vec::new();
    let mut bag = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(calories) = line {
                match calories.as_str() {
                    "" => {
                        elves.push(bag);
                        bag = Vec::new();
                    }
                    _ => bag.push(calories.parse().unwrap()),
                }
            }
        }
        // add the last bag.
        elves.push(bag);
    }
    
    let mut _most :u32 = 0;
    
    // sort the array
    elves.sort_by(|a:&Bag,b:&Bag| b.total_calories().cmp(&a.total_calories()));

    // print out calories
    for bag in &elves{
      println!("{}", bag.total_calories());
    }

    let top3 = &elves[0..3];

    println!("{:?}", &elves[0..3]);

    let mut total = 0;
    for bag in top3{
      total += bag.total_calories();
    }
    println!("total: {total}");

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fn type_of<T>(_: T) -> &'static str {
//     std::any::type_name::<T>()
// }
