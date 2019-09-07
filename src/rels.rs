use crate::wfc::{Item, Relation};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir2D {
    Up,
    Left,
    Right,
    Down,
}

impl Dir2D {
    pub fn step(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        use Dir2D::*;
        let v = match self {
            Right => (x + 1, y),
            Left if x == 0 => return None,
            Left => (x - 1, y),

            Up => (x, y + 1),
            Down if y == 0 => return None,
            Down => (x, y - 1),
        };
        Some(v)
    }
    pub fn iter(x: usize, y: usize) -> Vec<(Dir2D, (usize, usize))> {
        use Dir2D::*;
        vec![Up, Down, Left, Right]
            .drain(..)
            .filter_map(|d| d.step(x, y).map(|loc2d| (d, loc2d)))
            .collect()
    }
    pub fn opp(&self) -> Self {
        use Dir2D::*;
        match self {
            Up => Down,
            Down => Up,

            Right => Left,
            Left => Right,
        }
    }
}

impl<T: Item> Relation for (T, Dir2D) {
    type Loc = (usize, usize);
    type Item = T;
    fn add_related(&self, at: &(usize, usize), into: &mut HashMap<Self::Loc, HashSet<T>>) {
        let &(x, y) = at;
        let &(v, dir) = self;
        dir.step(x, y)
            .map(|pos| into.entry(pos).or_insert_with(HashSet::new).insert(v));
    }
}

pub fn get_2d_rels<T: Item>(items: &Vec<Vec<T>>) -> HashMap<T, HashSet<(T, Dir2D)>> {
    let mut out = HashMap::new();
    items.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, i)| {
            let mut allowed = HashSet::new();

            Dir2D::iter(x, y).drain(..).for_each(|(dir, (nx, ny))| {
                items
                    .get(ny)
                    .and_then(|row| row.get(nx))
                    .map(|&i| allowed.insert((i, dir)));
            });

            out.entry(*i).or_insert_with(HashSet::new).extend(allowed);
        })
    });
    out
}

impl<T: Item> Relation for (T, (i32, i32)) {
    type Loc = (usize, usize);
    type Item = T;
    fn add_related(&self, at: &(usize, usize), into: &mut HashMap<Self::Loc, HashSet<T>>) {
        use std::convert::TryFrom;
        use std::usize;
        let &(x, y) = at;
        let (dx, dy) = self.1;
        usize::try_from(dx + (x as i32))
            .and_then(|ox| usize::try_from(dy + (y as i32)).map(|oy| (ox, oy)))
            .ok()
            .map(|pos| {
                into.entry(pos).or_insert_with(HashSet::new).insert(self.0);
            });
    }
}
