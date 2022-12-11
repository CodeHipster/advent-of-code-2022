use std::collections::VecDeque;
use std::fs;
use std::str;
use std::time::Instant;

use itertools::Itertools;

use crate::monkey::*;

pub mod monkey;

fn main() {
    let now = Instant::now();

    let file = read_file("input.txt");
    let monkeys = file
        .lines()
        .tuples()
        .map(|(_, items, operation, test, true_friend, false_friend, _)| {
            let items = items[18..].split(",").map(|item| item.trim().parse::<u32>().unwrap()).collect::<VecDeque<_>>();
            let operation = operation[19..]
                .split(" ")
                .tuples()
                .map(|(left, op, right)| {
                    // match can't return different types, and each closure has a differen type
                    // so we will need to box the closure.
                    // also, we need to assign it to a value to be able to define the type.
                    // also we need to take ownership of the value, using the 'move' keyword
                    // also we can't use the matched value, because we can only match the &str in a tuple.
                    let right = right.to_owned();
                    let o: Box<dyn Fn(u32) -> u32> = match (left, op, right.as_str()) {
                        ("old", "*", "old") => Box::new(|old: u32| old * old),
                        ("old", "+", "old") => Box::new(|old: u32| old + old),
                        ("old", "*", _) => Box::new(move |old: u32| old * right.parse::<u32>().unwrap()),
                        ("old", "+", _) => Box::new(move |old: u32| old + right.parse::<u32>().unwrap()),
                        _ => panic!("unexpected match."),
                    };
                    o
                })
                .next()
                .unwrap(); // there is no more operations :)
            let test = test[21..].parse::<u32>().unwrap();
            let true_friend = true_friend[29..].parse::<usize>().unwrap();
            let false_friend = false_friend[30..].parse::<usize>().unwrap();
            Monkey::new(items, operation, test, true_friend, false_friend)
        })
        .collect::<Vec<Monkey>>();

    let answer = do_business(monkeys);

    println!("found answer: {answer} in {:0.2?}", now.elapsed());
}

fn do_business(mut monkeys: Vec<Monkey>) -> u32 {
    // for 20 rounds
    for _ in 0..20 {
        // for each monkey
        for i in 0..monkeys.len() {
          println!("Monkey {i}:");
            // can only have a reference to the monkey, as the size is not known at compile time
            // probably due to the box. (which will make it slow since cpu caches aren't used?)
            let m = &monkeys[i];
            // for all items
            let nr_items = m.items.borrow().len(); // have to move this out of the for loop, else the for loop would borrow the items reference.
            for j in 0..nr_items{ // todo this can be an iter.
              // pick the item
              let item = m.items.borrow_mut().pop_front().unwrap();
              println!("  Monkey inspects an item with a worry level of {item}.");
              let item = (m.operation)(item);
              println!("    Worry level is changed to {item}.");
              let item = item/3;
              println!("    Monkey gets bored with item. Worry level is divided by 3 to {item}.");
              if item % m.test == 0{
                println!("    Current worry level is divisible by {}.", m.test);
                println!("    Item with worry level {item} is thrown to monkey {}.",m.true_friend);
                //throw to true_friend
                // !!!! cannot borrow the Vec as mutable again...
                // !!!! we need to separate the data we are mutating and viewing?
                monkeys[m.true_friend].items.borrow_mut().push_back(item);
              }else{
                println!("    Current worry level is not divisible by {}.", m.test);
                println!("    Item with worry level {item} is thrown to monkey {}.",m.false_friend);
                //throw to false_friend
                monkeys[m.false_friend].items.borrow_mut().push_back(item);
              }
            }
            let mut m = &mut monkeys[i];
            m.business += nr_items as u32;
            // decide where to throw it
        }
    }

    monkeys.sort_by(|a, b| b.business.cmp(&a.business));

    return monkeys[0].business * monkeys[1].business;
}

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    }
}
