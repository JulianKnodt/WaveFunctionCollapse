use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Grid<T>(usize, Vec<T>);

impl<T> Grid<T> {
    pub fn new(s: usize, mut items: HashMap<(usize, usize), T>) -> Self {
        let mut temp = vec![];
        (0..(s * s)).for_each(|_| temp.push(None));
        let uniq_locs = items
            .drain()
            .all(|((x, y), i)| temp[x + y * s].replace(i).is_none());
        assert!(uniq_locs);
        Grid(s, temp.into_iter().map(|i| i.unwrap()).collect())
    }
}

impl<T: Debug> Grid<T> {
    pub fn display(&self) {
        self.1.chunks(self.0).for_each(|row| {
            row.iter().for_each(|i| print!("{:?} ", i));
            println!();
        });
    }
}

impl<T: Debug> Grid<Option<T>> {
    pub fn display_partial(&self) {
        self.1.chunks(self.0).for_each(|row| {
            row.iter().for_each(|i| {
                if let Some(v) = i {
                    print!("{:?} ", v)
                } else {
                    print!("? ")
                }
            });
            println!();
        });
    }
}
