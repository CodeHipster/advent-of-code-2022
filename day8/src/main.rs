use std::collections::HashMap;
use std::fs;
use std::str;
use std::time::Instant;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Tree {
    x: usize,
    y: usize,
    height: i32,
}

fn main() {
    let now = Instant::now();

    let mut scores: HashMap<&Tree, u32> = HashMap::new();

    let grid = read_file("input.txt")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Tree {
                    x,
                    y,
                    height: i32::try_from(c.to_digit(10).unwrap()).unwrap(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // go through rows left to right
    println!("rows:");
    let height = grid.len();
    for row in &grid {
        let width = row.len();
        for tree in row {
            // calculate score //TODO: extract to method.
            // right
            let mut score = 0;
            for x in tree.x..width {
                let tree_line = grid.get(tree.y).unwrap();
                let comp = tree_line.get(x).unwrap();
                if tree == comp {
                    continue;
                } else if tree.height > comp.height {
                    score += 1;
                    continue;
                } else if tree.height <= comp.height {
                    score += 1;
                    break;
                }
            }
            *scores.entry(tree).or_insert(1) *= score;
            // left
            let mut score = 0;
            for x in (0..tree.x).rev() {
                let tree_line = grid.get(tree.y).unwrap();
                let comp = tree_line.get(x).unwrap();
                if tree == comp {
                    continue;
                } else if tree.height > comp.height {
                    score += 1;
                } else if tree.height <= comp.height {
                    score += 1;
                    break;
                }
            }
            *scores.entry(tree).or_insert(1) *= score;
            // top
            let mut score = 0;
            for y in (0..tree.y).rev() {
                let tree_line = grid.get(y).unwrap();
                let comp = tree_line.get(tree.x).unwrap();
                if tree == comp {
                    continue;
                } else if tree.height > comp.height {
                    score += 1;
                } else if tree.height <= comp.height {
                    score += 1;
                    break;
                }
            }
            *scores.entry(tree).or_insert(1) *= score;
            // bottom
            let mut score = 0;
            for y in (tree.y..height) {
                let tree_line = grid.get(y).unwrap();
                let comp = tree_line.get(tree.x).unwrap();
                if tree == comp {
                    continue;
                } else if tree.height > comp.height {
                    score += 1;
                } else if tree.height <= comp.height {
                    score += 1;
                    break;
                }
            }
            *scores.entry(tree).or_insert(1) *= score;
        }
    }

    let mut answer = scores.iter().map(|(_,v)| v).max();
    println!("found answer: {answer:?}, in {:0.2?}", now.elapsed());
}

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    }
}
