use std::collections::HashMap;
use std::fs;
use std::str;
use std::time::Instant;

fn main() {
  let now = Instant::now();

  let file = read_file("input.txt");
  let sum = file
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| match c {
          '=' => -2 as i64,
          '-' => -1 as i64,
          '0' => 0 as i64,
          '1' => 1 as i64,
          '2' => 2 as i64,
          _ => panic!("unexpected char."),
        })
        .collect::<Vec<_>>()
    })
    .map(|snafu| snafu.iter().rev().enumerate().map(|(i, v)| 5_i64.pow(i as u32) * v).sum::<i64>())
    .sum::<i64>();

  println!("decimal sum: {sum}");

  let answer = to_snafu(sum);

  println!("found answer: {} in {:0.2?}", answer, now.elapsed());
}

fn read_file(filename: &str) -> String {
  match fs::read_to_string(filename) {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    }
  }
}

fn to_snafu(nr: i64)-> String{
  let mut min_max_map : Vec<(i64, i64)> = Vec::new();

  let mut index = 0;

  loop {
    let max = 5_i64.pow(min_max_map.len() as u32) * 2;
    let min = max * -1;
    min_max_map.push((min, max));
    if nr.abs() < max {
      break;
    }
    index += 1;
  }

  let mut to_go = nr;
  let mut snafu = "".to_owned();
  for (exp, min_max) in  min_max_map.iter().enumerate().rev(){
    let mut closest_index = (0, i64::MAX);
    for (index, mul) in (-2..=2).enumerate(){
      let result = to_go-(mul*(5_i64.pow(exp as u32)));
      if result.abs() < closest_index.1.abs(){
        closest_index = (mul, result);
      }
    }
    let snaf = match closest_index.0{
      -2 => '=',
      -1 => '-',
      0 => '0',
      1 => '1',
      2 => '2',
      _ => panic!("unexpected")
    };
    to_go = closest_index.1;
    snafu = format!("{snafu}{snaf}");
    println!("{snafu}");
  }

  snafu
}
