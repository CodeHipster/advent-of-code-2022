use grid::Grid;

#[derive(Clone)]
pub enum TileType{
  Void,
  Rock,
  Open,
}

pub struct Location{
  row: usize,
  col: usize,
}

pub enum Direction{
  Up, Down, Left, Right,
}

pub enum TurnType{
  Left, Right
}

pub enum Action{
  Move(usize),
  Turn(TurnType),
}

pub struct Pawn{
  loc: Location,
  face: Direction,
  actions: Vec<Action>
}

pub struct Map{
  tiles: Grid<TileType>
}

