use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub struct Monkey {
    pub items: RefCell<VecDeque<u32>>,
    pub business: u32,
    pub operation: Box<dyn Fn(u32) -> u32>,
    pub test: u32,
    pub true_friend: usize,
    pub false_friend: usize,
}

impl Monkey {
    pub fn new(items: VecDeque<u32>, operation: Box<dyn Fn(u32) -> u32>, test: u32, friend_true: usize, friend_false: usize) -> Monkey {
        Monkey {
            items: RefCell::new(items),
            business: 0,
            operation,
            test,
            true_friend: friend_true,
            false_friend: friend_false,
        }
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "items: {:?}, business: {}, test: {}, true_friend: {}, false_friend: {}",
            self.items, self.business, self.test, self.true_friend, self.false_friend
        )
    }
}
