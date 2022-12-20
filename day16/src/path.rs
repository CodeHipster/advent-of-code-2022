use std::collections::{HashMap};

// TODO: have f return a reference to a vector
// TODO: make generic instead of fixed u16
// return a map with the shortest distance to all other nodes.
pub fn distance_map<F>(start: u16, f: F) -> HashMap<u16, usize>
where
  F: Fn(u16) -> Vec<u16>,
{
  let mut visited: HashMap<u16, usize> = HashMap::new();
  // insert start as visited.
  visited.insert(start, 0);
  // Using a vec makes it a depth first search (as it is used as a stack, so last in first out)
  let mut stack: Vec<(u16, usize)> = vec![(start, 0)];

  while !stack.is_empty() {
    let (node, hops) = stack.pop().unwrap();
    let hops = hops + 1; // Set to nr of hops to neighbours.
    let neighbours = f(node);
    for n in neighbours {
      let v = visited.get(&n);
      if let Some(n_hops) = v{
        // neighbour has been visited before.
        if *n_hops <= hops {
          // previous route was smaller, stop chasing this path.
          continue;
        }
      }
      // this route is faster, add to stack.
      visited.insert(n, hops);
      stack.push((n, hops));
    }
  }
  return visited;
}
