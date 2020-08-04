use crate::wfc::Relation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Dir2D {
  Up,
  Left,
  Right,
  Down,
}

pub const ALL_DIRS: [Dir2D; 4] = [Dir2D::Up, Dir2D::Down, Dir2D::Left, Dir2D::Right];

impl Dir2D {
  pub fn step(self, x: usize, y: usize) -> Option<(usize, usize)> {
    use Dir2D::*;
    let v = match self {
      Left if x == 0 => return None,
      Down if y == 0 => return None,

      Right => (x + 1, y),
      Left => (x - 1, y),
      Up => (x, y + 1),
      Down => (x, y - 1),
    };
    Some(v)
  }
  pub fn iter(x: usize, y: usize) -> impl Iterator<Item = (Dir2D, (usize, usize))> {
    ALL_DIRS
      .iter()
      .copied()
      .filter_map(move |d| d.step(x, y).map(|loc2d| (d, loc2d)))
  }
  pub fn opp(self) -> Self {
    use Dir2D::*;
    match self {
      Up => Down,
      Down => Up,

      Right => Left,
      Left => Right,
    }
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Neighbor2D<T> {
  allowed: T,
  direction: Dir2D,
}

impl<T> Neighbor2D<T> {
  pub fn new(allowed: T, direction: Dir2D) -> Self { Self { allowed, direction } }
}

impl<T: Copy + std::fmt::Debug> Relation for Neighbor2D<T> {
  type Loc = (usize, usize);
  type Item = T;

  fn related(&self, (x, y): Self::Loc, mut f: impl FnMut(Self::Loc, Self::Item)) {
    if let Some(p) = self.direction.step(x, y) {
      f(p, self.allowed)
    }
  }
}

pub fn get_2d_rels<T: Copy>(
  items: &[Vec<T>],
) -> impl Iterator<Item = (T, Vec<Neighbor2D<T>>)> + '_ {
  items.iter().enumerate().flat_map(move |(y, row)| {
    row.iter().enumerate().map(move |(x, &i)| {
      let dirs = Dir2D::iter(x, y)
        .filter_map(move |(dir, (nx, ny))| {
          items
            .get(ny)
            .and_then(|row| row.get(nx))
            .map(|&i| Neighbor2D::new(i, dir))
        })
        .collect();
      (i, dirs)
    })
  })
}

/*
impl<T: Item> Relation for (T, (i32, i32)) {
  type Loc = (usize, usize);
  type Item = T;
  fn add_related(&self, at: &(usize, usize), into: &mut HashMap<Self::Loc, HashSet<T>>) {
    use std::{convert::TryFrom, usize};
    let &(x, y) = at;
    let (dx, dy) = self.1;
    if let Ok(pos) = usize::try_from(dx + (x as i32))
      .and_then(|ox| usize::try_from(dy + (y as i32)).map(|oy| (ox, oy)))
    {
      into.entry(pos).or_insert_with(HashSet::new).insert(self.0);
    }
  }
}

impl<T: Item> Relation for (T, (i32, i32, i32)) {
  type Loc = (usize, usize, usize);
  type Item = T;
  fn add_related(&self, at: &(usize, usize, usize), into: &mut HashMap<Self::Loc, HashSet<T>>) {
    use std::{convert::TryFrom, usize};
    let &(x, y, z) = at;
    let (dx, dy, dz) = self.1;
    usize::try_from(dx + (x as i32))
      .and_then(|ox| usize::try_from(dy + (y as i32)).map(|oy| (ox, oy)))
      .and_then(|(ox, oy)| usize::try_from(dz + (z as i32)).map(|oz| (ox, oy, oz)))
      .ok()
      .map(|pos| into.entry(pos).or_insert_with(HashSet::new).insert(self.0));
  }
}
*/
