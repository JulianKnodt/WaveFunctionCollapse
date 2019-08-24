use crate::wfc::{Item, Relation};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir2D<T> {
    Up(T),
    Left(T),
    Right(T),
    Down(T),
}

fn left(x: usize, y: usize) -> Option<(usize, usize)> {
    if x != 0 {
        Some((x - 1, y))
    } else {
        None
    }
}
fn right(x: usize, y: usize) -> (usize, usize) {
    (x + 1, y)
}
fn up(x: usize, y: usize) -> (usize, usize) {
    (x, y + 1)
}
fn down(x: usize, y: usize) -> Option<(usize, usize)> {
    if y != 0 {
        Some((x, y - 1))
    } else {
        None
    }
}

impl<T: Item> Relation for Dir2D<T> {
    type Loc = (usize, usize);
    type Item = T;
    fn related(&self, _to: &T, at: &(usize, usize)) -> HashMap<Self::Loc, HashSet<T>> {
        use Dir2D::*;
        let mut out = HashMap::new();
        let &(x, y) = at;
        let (pos, v) = match self {
            Up(v) => (Some(up(x, y)), v),
            Left(v) => (left(x, y), v),
            Right(v) => (Some(right(x, y)), v),
            Down(v) => (down(x, y), v),
        };
        pos.map(|pos| {
            let mut just = HashSet::new();
            just.insert(*v);
            out.insert(pos, just);
        });
        out
    }
}

pub fn get_2d_rels<T: Item>(items: &Vec<Vec<T>>) -> HashMap<T, HashSet<Dir2D<T>>> {
    let mut out = HashMap::new();
    items.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, i)| {
            let mut allowed = HashSet::new();

            use Dir2D::*;
            let (ux, uy) = up(x, y);
            items
                .get(uy)
                .and_then(|row| row.get(ux))
                .map(|&i| allowed.insert(Up(i)));

            down(x, y).map(|(dx, dy)| {
                items
                    .get(dy)
                    .and_then(|row| row.get(dx))
                    .map(|&i| allowed.insert(Down(i)))
            });

            left(x, y).map(|(lx, ly)| {
                items
                    .get(ly)
                    .and_then(|row| row.get(lx))
                    .map(|&i| allowed.insert(Left(i)))
            });

            let (rx, ry) = right(x, y);
            items
                .get(ry)
                .and_then(|row| row.get(rx))
                .map(|&i| allowed.insert(Right(i)));

            out.entry(*i)
                .or_insert_with(|| HashSet::new())
                .extend(allowed);
        })
    });
    out
}

#[test]
fn get_2d_rels_test() {
    use Dir2D::*;
    let examples = vec![
        vec![0, 1, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 0],
    ];
    let rels = get_2d_rels(&examples);
    println!("{:?}", rels);
    assert!(rels[&0].contains(&Left(0)));
    assert!(rels[&0].contains(&Right(0)));
    assert!(rels[&0].contains(&Up(0)));
    assert!(rels[&0].contains(&Down(0)));

    assert!(rels[&0].contains(&Left(1)));
    assert!(rels[&0].contains(&Right(1)));
    assert!(rels[&0].contains(&Up(1)));
    assert!(rels[&0].contains(&Down(1)));

    assert!(rels[&1].contains(&Left(0)));
    assert!(rels[&1].contains(&Right(0)));
    assert!(rels[&1].contains(&Up(0)));
    assert!(rels[&1].contains(&Down(0)));
}
