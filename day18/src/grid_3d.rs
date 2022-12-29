// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use core::ops::Add;

/// Representation of a point in three dimensions
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Point3D {
  pub x: i32,
  pub y: i32,
  pub z: i32,
}

impl Default for Point3D {
  fn default() -> Self {
    Self { x: 0, y: 0, z: 0 }
  }
}

impl Add for Point3D {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

/// Trait for implementing a Grid in three dimensions
pub trait Grid3D {
  /// The dimensions of the Grid. The only method that must be defined.
  fn dimensions(&self) -> Point3D;

  /// The lower bound. Defaults to (0, 0, 0).
  fn lower_bound(&self) -> Point3D {
    Default::default()
  }

  /// The upper bound. Defaults to the dimensions itself.
  fn upper_bound(&self) -> Point3D {
    let dim = self.dimensions();
    let low = self.lower_bound();
    Point3D {
      x: low.x + dim.x,
      y: low.y + dim.y,
      z: low.z + dim.z,
    }
  }

  /// Check if a point is in the bounds of the grid.
  fn in_bounds(&self, point: Point3D) -> bool {
    let low = self.lower_bound();
    let upp = self.upper_bound();
    low.x <= point.x && point.x < upp.x && low.y <= point.y && point.y < upp.y && low.z <= point.z && point.z < upp.z
  }

  /// Convert a point to an index.
  ///
  /// Useful if you store the grid in a one-dimensional array.
  fn point3d_to_index(&self, point: Point3D) -> usize {
    let dim = self.dimensions();
    (point.z * dim.y * dim.x + point.y * dim.x + point.x) as usize
  }

  /// Convert an index to a point.
  ///
  /// Useful if you store the grid in a one-dimensional array.
  fn index_to_point3d(&self, index: usize) -> Point3D {
    let dim = self.dimensions();
    let mut idx = index as i32;
    let z = idx / (dim.x * dim.y);
    idx -= z * dim.x * dim.y;
    let y = idx / dim.x;
    let x = idx % dim.x;
    Point3D { x, y, z }
  }

  /// Check if a point is traversable.
  ///
  /// Defaults to always, so you may want to implement
  /// this.
  #[allow(unused_variables)]
  fn can_traverse(&self, point: Point3D) -> bool;

  /// Get all possible neighbors of the point, regardless if the
  /// point or its neighbors is in bounds, opaque, or neither.
  fn get_possible_neighbors(&self, point: Point3D) -> [Point3D; 6] {
    [
      point + Point3D { x: 0, y: 1, z: 0 },
      point + Point3D { x: 0, y: -1, z: 0 },
      point + Point3D { x: 1, y: 0, z: 0 },
      point + Point3D { x: -1, y: 0, z: 0 },
      point + Point3D { x: 0, y: 0, z: 1 },
      point + Point3D { x: 0, y: 0, z: -1 },
    ]
  }

  /// Check if two points are possible neighbors.
  ///
  /// Does not check if either points are inbounds or non-opaque.
  fn is_possible_neighbor(&self, p1: Point3D, p2: Point3D) -> bool {
    self.get_possible_neighbors(p1).iter().any(|&x| x == p2)
  }

  /// Get the neighbors that is in bounds and not opaque.
  fn get_neighbors(&self, point: Point3D) -> [Option<Point3D>; 6] {
    let mut arr: [Option<Point3D>; 6] = [None; 6];
    let possible_neighbors = self.get_possible_neighbors(point);
    for (i, n) in possible_neighbors.iter().enumerate() {
      if self.in_bounds(*n) && self.can_traverse(*n) {
        arr[i] = Some(*n);
      }
    }
    arr
  }

  /// Check if two points are neighbors.
  ///
  /// Checks if either points are inbounds or non-opaque.
  fn is_neighbor(&self, p1: Point3D, p2: Point3D) -> bool {
    self.get_neighbors(p1).iter().any(|&x| x == Some(p2))
  }

}
