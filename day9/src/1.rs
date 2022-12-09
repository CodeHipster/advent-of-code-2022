use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::str;
use std::time::Instant;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Pos {
  x:i32,
  y:i32,
}

struct Rope{
  head:Pos,
  tail:Pos,
}

struct Move<'a>{
  tr: &'a Pos, //translation
  n : u8,
}

#[derive(Eq, Hash, PartialEq, Debug)]
enum Dir{
  U,D,L,R
}

impl Pos{
  fn add(&mut self, other: &Pos){
    self.x += other.x;
    self.y += other.y;
  }
  fn sub(& self, other: &Pos)-> Pos{
    Pos{x: self.x - other.x, y: self.y - other.y}
  }
}

fn main() {
    let now = Instant::now();

    let mut positions: HashSet<Pos> = HashSet::new();
    let mut rope = Rope{head:Pos{x:0,y:0},tail:Pos{x:0,y:0}};

    let mut moves: HashMap<Dir, Pos> = HashMap::new();
    moves.insert(Dir::U,Pos{x:0, y:1});
    moves.insert(Dir::D,Pos{x:0, y:-1});
    moves.insert(Dir::L,Pos{x:-1, y:0});
    moves.insert(Dir::R,Pos{x:1, y:0});

    let grid = read_file("input.txt")
        .lines()
        .map(|line| line.split(" ").collect())
        .map(|words :Vec<&str>| 
          match words[..] {
              ["R", n] => Move{tr: moves.get(&Dir::R).unwrap(),n: n.parse::<u8>().unwrap()},
              ["L", n] => Move{tr: moves.get(&Dir::L).unwrap(),n: n.parse::<u8>().unwrap()},
              ["U", n] => Move{tr: moves.get(&Dir::U).unwrap(),n: n.parse::<u8>().unwrap()},
              ["D", n] => Move{tr: moves.get(&Dir::D).unwrap(),n: n.parse::<u8>().unwrap()},
              [] | [_] | [_, ..] => panic!("unexpected match."),
        })
        .for_each(|mv| move_rope(&mv, &mut rope, &mut positions));
        

    println!("found answer: {:?}, in {:0.2?}",positions.len(), now.elapsed());
}

fn move_rope(mv: &Move, rope: &mut Rope, positions: &mut HashSet<Pos>){
  // loop n times
  for _ in 0..mv.n {
    //move rope head
    rope.head.add(mv.tr);
    //find the diff
    let diff = rope.head.sub(&rope.tail);
    //move tail according to diff
    // if x|y absolute > 2, we need to move
    if diff.x == 2 {// move right
      if diff.y == 0 {// right
        rope.tail.add(&Pos{x:1,y:0})
      }else if diff.y == 1{ // right up
        rope.tail.add(&Pos{x:1,y:1})
      }else if diff.y == -1{ // right down
        rope.tail.add(&Pos{x:1,y:-1})
      }else{
        panic!("unexpected diff: {diff:?}");
      }
    } else if diff.x == -2 {// move left
      if diff.y == 0 {// right
        rope.tail.add(&Pos{x:-1,y:0})
      }else if diff.y == 1{ // right up
        rope.tail.add(&Pos{x:-1,y:1})
      }else if diff.y == -1{ // right down
        rope.tail.add(&Pos{x:-1,y:-1})
      }else{
        panic!("unexpected diff: {diff:?}");
      }
    } else if diff.y == 2 {// move up
      if diff.x == 0 {// up
        rope.tail.add(&Pos{x:0,y:1})
      }else if diff.x == 1{ // up right
        rope.tail.add(&Pos{x:1,y:1})
      }else if diff.x == -1{ // up left
        rope.tail.add(&Pos{x:-1,y:1})
      }else{
        panic!("unexpected diff: {diff:?}");
      }
    } else if diff.y == -2 {// move down
      if diff.x == 0 {// down
        rope.tail.add(&Pos{x:0,y:-1})
      }else if diff.x == 1{ // down right
        rope.tail.add(&Pos{x:1,y:-1})
      }else if diff.x == -1{ // down left
        rope.tail.add(&Pos{x:-1,y:-1})
      }else{
        panic!("unexpected diff: {diff:?}");
      }
    }else{
      // println!("not moving: {diff:?}");
    }

    //add tail pos to hashset
    positions.insert(Pos{x: rope.tail.x, y:rope.tail.y});

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
