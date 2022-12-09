use std::cmp::{max, min};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs;
use std::str;
use std::time::Instant;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rope {
    parts: Vec<RopePart>,
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //print the snake
        // find min and max positions
        let max = self.parts.iter().map(|part| max(max(part.head.x, part.head.y), max(part.tail.x, part.tail.y))).max().unwrap();
        let min = self.parts.iter().map(|part| min(min(part.head.x, part.head.y), min(part.tail.x, part.tail.y))).min().unwrap();

        // make a grid
        let offset = -min;
        let m = max + offset;
        let size = (m + 1) as usize;
        let mut grid = vec![vec![".".to_owned(); size]; size];
        println!("max: {max}, min: {min}, offset: {offset}, size: {size}, m: {m}");

        // add ropeparts to the grid
        let head = &self.parts[0].head;
        grid[(m - (head.y + offset)) as usize][(head.x + offset) as usize] = "H".to_owned();
        self.parts.iter().enumerate().for_each(|(index, part)| grid[(m - (part.tail.y + offset)) as usize][(part.tail.x + offset) as usize] = (index + 1).to_string());

        // print the grid
        for row in grid {
            for point in row {
                write!(f, "{point}")?
            }
            writeln!(f, "")?
        }

        Ok(())
    }
}

#[derive(Debug)]
struct RopePart {
    head: Pos,
    tail: Pos,
}

#[derive(Debug)]
struct Move<'a> {
    tr: &'a Pos, //translation
    n: u8,
}

#[derive(Eq, Hash, PartialEq, Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl Pos {
    fn add(&mut self, other: &Pos) {
        self.x += other.x;
        self.y += other.y;
    }
    fn sub(&self, other: &Pos) -> Pos {
        Pos { x: self.x - other.x, y: self.y - other.y }
    }
}

fn main() {
    let now = Instant::now();

    let mut positions: HashSet<Pos> = HashSet::new();
    let mut rope = Rope {
        parts: vec![
            RopePart { head: Pos { x: 0, y: 0 }, tail: Pos { x: 0, y: 0 } },
            RopePart { head: Pos { x: 0, y: 0 }, tail: Pos { x: 0, y: 0 } },
            RopePart { head: Pos { x: 0, y: 0 }, tail: Pos { x: 0, y: 0 } },
            RopePart { head: Pos { x: 0, y: 0 }, tail: Pos { x: 0, y: 0 } },
            RopePart { head: Pos { x: 0, y: 0 }, tail: Pos { x: 0, y: 0 } },
            RopePart { head: Pos { x: 0, y: 0 }, tail: Pos { x: 0, y: 0 } },
            RopePart { head: Pos { x: 0, y: 0 }, tail: Pos { x: 0, y: 0 } },
            RopePart { head: Pos { x: 0, y: 0 }, tail: Pos { x: 0, y: 0 } },
            RopePart { head: Pos { x: 0, y: 0 }, tail: Pos { x: 0, y: 0 } },
        ],
    };

    let mut moves: HashMap<Dir, Pos> = HashMap::new();
    moves.insert(Dir::U, Pos { x: 0, y: 1 });
    moves.insert(Dir::D, Pos { x: 0, y: -1 });
    moves.insert(Dir::L, Pos { x: -1, y: 0 });
    moves.insert(Dir::R, Pos { x: 1, y: 0 });

    read_file("input.txt")
        .lines()
        .map(|line| line.split(" ").collect())
        .map(|words: Vec<&str>| match words[..] {
            ["R", n] => Move {
                tr: moves.get(&Dir::R).unwrap(),
                n: n.parse::<u8>().unwrap(),
            },
            ["L", n] => Move {
                tr: moves.get(&Dir::L).unwrap(),
                n: n.parse::<u8>().unwrap(),
            },
            ["U", n] => Move {
                tr: moves.get(&Dir::U).unwrap(),
                n: n.parse::<u8>().unwrap(),
            },
            ["D", n] => Move {
                tr: moves.get(&Dir::D).unwrap(),
                n: n.parse::<u8>().unwrap(),
            },
            [] | [_] | [_, ..] => panic!("unexpected match."),
        })
        .for_each(|mv| move_rope(&mv, &mut rope, &mut positions));

    println!("found answer: {:?}, in {:0.2?}", positions.len(), now.elapsed());
}

fn move_rope(mv: &Move, rope: &mut Rope, positions: &mut HashSet<Pos>) {
    // loop n times
    for _ in 0..mv.n {
        //move rope head
        rope.parts[0].head.add(mv.tr);
        move_tail(&mut rope.parts[0]);
        // move each rope part
        for i in 1..rope.parts.len() {
            // quick hack to align knots
            rope.parts[i].head.x = rope.parts[i - 1].tail.x;
            rope.parts[i].head.y = rope.parts[i - 1].tail.y;
            move_tail(&mut rope.parts[i]);
        }

        //add tail pos to hashset
        positions.insert(Pos { x: rope.parts[8].tail.x, y: rope.parts[8].tail.y });
    }
}

fn move_tail(part: &mut RopePart) {
    //find the diff
    let diff = part.head.sub(&part.tail);
    //move tail according to diff
    // if x|y absolute > 2, we need to move
    if diff.x == 2 {
        // move right
        if diff.y == 0 {
            // right
            part.tail.add(&Pos { x: 1, y: 0 })
        } else if diff.y > 0 {
            // right up
            part.tail.add(&Pos { x: 1, y: 1 })
        } else if diff.y < 0 {
            // right down
            part.tail.add(&Pos { x: 1, y: -1 })
        }
    } else if diff.x == -2 {
        // move left
        if diff.y == 0 {
            // right
            part.tail.add(&Pos { x: -1, y: 0 })
        } else if diff.y > 0 {
            // right up
            part.tail.add(&Pos { x: -1, y: 1 })
        } else if diff.y < 0 {
            // right down
            part.tail.add(&Pos { x: -1, y: -1 })
        }
    } else if diff.y == 2 {
        // move up
        if diff.x == 0 {
            // up
            part.tail.add(&Pos { x: 0, y: 1 })
        } else if diff.x > 0 {
            // up right
            part.tail.add(&Pos { x: 1, y: 1 })
        } else if diff.x < 0 {
            // up left
            part.tail.add(&Pos { x: -1, y: 1 })
        }
    } else if diff.y == -2 {
        // move down
        if diff.x == 0 {
            // down
            part.tail.add(&Pos { x: 0, y: -1 })
        } else if diff.x > 0 {
            // down right
            part.tail.add(&Pos { x: 1, y: -1 })
        } else if diff.x < 0 {
            // down left
            part.tail.add(&Pos { x: -1, y: -1 })
        }
    }
}

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    }
}
