pub struct Mixer {
  // original index, nr
  nrs: Vec<(usize,i16)>,
}

fn new_index(current: usize, nr: i16, length: usize) -> usize {
  let mut index = (current as i32 + nr as i32) % length as i32;
  if index <= 0 {
    index += length as i32;
  }
  index as usize
}

impl Mixer {
  pub fn new(nrs: Vec<i16>) -> Mixer {
    let mut indexed = vec![];
    for (index,nr) in nrs.iter().enumerate(){
      indexed.push((index, *nr))
    }
    Mixer { nrs:indexed }
  }

  pub fn mix(&mut self) {
    let length = self.nrs.len();
    for i in 0..length {
      let index = self.nrs.iter().position(|&r| r.0 == i).unwrap();
      let nr = self.nrs.remove(index);
      let new_index = new_index(index, nr.1, length - 1);
      self.nrs.insert(new_index, nr)
    }
    println!("mixed: {:?}", self.nrs);
  }

  pub fn answer(&self) -> i16 {
    // index 1000, 2000, 3000 added.
    let length = self.nrs.len();
    let index = self.nrs.iter().position(|&r| r.1 == 0).unwrap();
    let mut answer = self.nrs[(index + 1000) % length].1;
    answer += self.nrs[(index + 2000) % length].1;
    answer += self.nrs[(index + 3000) % length].1;
    answer
  }
}
