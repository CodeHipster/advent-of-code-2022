use std::collections::HashSet;
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

    let mut columns: Vec<Vec<Tree>> = vec![];

    let rows = read_file("input.txt")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let tree = Tree {
                        x,
                        y,
                        height: i32::try_from(c.to_digit(10).unwrap()).unwrap(),
                    };
                    // push the tree also in the columns
                    if y == 0 {
                        columns.push(vec![Tree {
                            x,
                            y,
                            height: i32::try_from(c.to_digit(10).unwrap()).unwrap(),
                        }])
                    } else {
                        columns
                            .get_mut(x)
                            .expect("expect column to exist.")
                            .push(Tree {
                                x,
                                y,
                                height: i32::try_from(c.to_digit(10).unwrap()).unwrap(),
                            })
                    }
                    tree
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut visible: HashSet<&Tree> = HashSet::new();

    // go through rows left to right
    println!("rows:");
    for row in &rows {
        let mut heighest = -1i32;
        for tree in row {
            // println!("checking tree: {tree:?}");
            if tree.height > heighest {
                visible.insert(tree);
                println!("tree is visible: {tree:?}");
                heighest = tree.height;
            }
        }
    }
    // go through rows right to left
    println!("rows reverse:");
    for row in &rows {
        let mut heighest = -1;
        for tree in row.iter().rev() {
            // println!("checking tree: {tree:?}");
            if tree.height > heighest {
                visible.insert(tree);
                println!("tree is visible: {tree:?}");
                heighest = tree.height;
            }
        }
    }

    // go through columns top to bottom
    println!("columns:");
    for column in &columns {
        let mut heighest = -1;
        for tree in column {
            // println!("checking tree: {tree:?}");
            if tree.height > heighest {
                visible.insert(tree);
                println!("tree is visible: {tree:?}");
                heighest = tree.height;
            }
        }
    }

    // go through columns bottom to top
    println!("columns reverse:");
    for column in &columns {
        let mut heighest = -1;
        for tree in column.iter().rev() {
            // println!("checking tree: {tree:?}");
            if tree.height > heighest {
                visible.insert(tree);
                println!("tree is visible: {tree:?}");
                heighest = tree.height;
            }
        }
    }

    println!("found answer: {}, in {:0.2?}",visible.len(), now.elapsed());
}

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    }
}
