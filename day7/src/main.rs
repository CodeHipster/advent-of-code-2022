use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;
use std::time::Instant;

#[derive(Debug)]
enum ScreenLine {
    Command(Command),
    File(u64, String),
    Directory(String),
}

#[derive(Debug)]
enum Command {
    Root,
    MoveIn(String),
    MoveUp,
    List,
}

fn main() {
    let now = Instant::now();

    let mut directories: HashMap<String, u64> = HashMap::new();
    let mut stack: Vec<String> = vec![];

    read_lines("input.txt")
        .map(|result| result.expect("Expect a line."))
        .map(|line| {
            let words: Vec<&str> = line.split(" ").collect();
            match words[..] {
                ["$", "cd", "/"] => ScreenLine::Command(Command::Root),
                ["$", "cd", ".."] => ScreenLine::Command(Command::MoveUp),
                ["$", "ls"] => ScreenLine::Command(Command::List),
                ["$", "cd", dir] => ScreenLine::Command(Command::MoveIn(dir.to_owned())),
                ["dir", dir] => ScreenLine::Directory(dir.to_owned()),
                [size, file] => ScreenLine::File(size.parse::<u64>().unwrap(), file.to_owned()),
                [] | [_] | [_, ..] => panic!("unexpected match."),
            }
        })
        .for_each(|line| {
            // operate directory stack
            // for each dir add to hashmap if not exists
            // for each file add size to all directories in stack
            match line {
                ScreenLine::Command(Command::Root) => {
                    stack.clear();
                    stack.push("/".to_owned())
                }
                ScreenLine::Command(Command::MoveIn(dir)) => {
                    let mut dir_name = stack.last().unwrap().to_owned();
                    dir_name.push_str(&dir);
                    dir_name.push_str("/");
                    stack.push(dir_name); //contains e.g. [/, /a/]
                }
                ScreenLine::Command(Command::MoveUp) => {
                    stack.pop();
                }
                ScreenLine::Command(Command::List) => (),
                ScreenLine::Directory(dir) => {
                    // add to hashmap
                    let mut dir_name = stack.last().unwrap().to_owned();
                    dir_name.push_str(&dir);
                    dir_name.push_str("/");
                    directories.entry(dir_name).or_insert(0);
                }
                ScreenLine::File(size, _) => {
                    // for each directory in the stack, add the size
                    for dir in &stack {
                        *directories.entry(dir.to_owned()).or_insert(0) += size;
                    }
                }
            }
        });


    let total_space = 70000000u64;
    let required = 30000000u64;
    let total_directory = directories.get("/").unwrap().to_owned();
    let to_be_deleted = total_directory - (total_space - required);

    println!("tbd: {to_be_deleted}, dir: {total_directory}");

    let mut smallest = total_directory;
    // go over directories. where size >= 100.000
    for (_, size) in &directories {
        if *size >= to_be_deleted && *size < smallest{
          smallest = *size;
        }
    }

    // println!("directories: {directories:?}");
    println!("found answer: {smallest:?}, in {:0.2?}", now.elapsed());
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
