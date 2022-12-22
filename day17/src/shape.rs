use grid::{grid, Grid};

pub struct Shape {
  pub grid: Grid<bool>,
  pub points: Vec<(i32, i32)>,
}

impl Shape {
  fn points(grid: &Grid<bool>) -> Vec<(i32, i32)>{
    let mut points = vec![];
    for r in 0..grid.rows(){
      for c in 0..grid.cols(){
        if grid[r][c]{
          // points are viewed from the left bottom, so rows are transposed.
          points.push((r as i32 - (grid.rows() as i32 -1 ), c as i32));
        }
      }
    }
    points
  }

  pub fn line() -> Shape {
    let grid = grid![[true, true, true, true]];
    let points = Shape::points(&grid);
    Shape { grid, points }
  }
  pub fn plus() -> Shape {
    let grid = grid![
    [false, true, false]
    [true, true, true]
    [false, true, false]
    ];
    let points = Shape::points(&grid);
    Shape { grid, points }
  }
  pub fn il() -> Shape {
    let grid = grid![
    [false, false, true]
    [false, false, true]
    [true, true, true]
    ];
    let points = Shape::points(&grid);
    Shape { grid, points }
  }
  pub fn tower() -> Shape {
    let grid = grid![[true][true][true][true]];
    let points = Shape::points(&grid);
    Shape { grid, points }
  }
  pub fn square() -> Shape {
    let grid = grid![
    [true, true]
    [true, true]
    ];
    let points = Shape::points(&grid);
    Shape { grid, points }
  }
}