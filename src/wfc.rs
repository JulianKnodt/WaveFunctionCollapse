use rand::prelude::*;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

pub trait Location: Eq + Hash + Copy + Debug {}
impl<T> Location for T where T: Eq + Hash + Copy + Debug {}

pub trait Item: Eq + Hash + Copy + Debug {}
impl<T> Item for T where T: Eq + Hash + Copy + Debug {}

pub trait Relation: Hash + Eq + Debug {
    type Loc: Location;
    type Item: Item;
    // returns a Vector of (location to update, allowed items)
    // for this relation
    fn related(&self, at: &Self::Loc) -> HashMap<Self::Loc, HashSet<Self::Item>>;
}

/// WaveFunctionCollapse maintains state for the wave function collapse function
#[derive(Debug)]
pub struct WaveFunctionCollapse<Loc: Location, T: Item, Rel: Relation<Loc = Loc, Item = T>> {
    possibilities: HashMap<Loc, HashSet<T>>,
    relations: HashMap<T, HashSet<Rel>>,
    frequencies: HashMap<T, f64>,
}

impl<Loc, T, Rel> WaveFunctionCollapse<Loc, T, Rel>
where
    Loc: Location,
    T: Item,
    Rel: Relation<Loc = Loc, Item = T>,
{
    /// Returns a new WFC with the items and relations given
    pub fn new(locs: Vec<Loc>, items: Vec<T>, relations: HashMap<T, HashSet<Rel>>) -> Self {
        let uniq: HashSet<_> = items.iter().map(|&i| i).collect();
        let possibilities: HashMap<_, _> = locs.iter().map(|l| (*l, uniq.clone())).collect();
        let mut frequencies = HashMap::new();
        items
            .iter()
            .for_each(|item| *frequencies.entry(*item).or_insert(0.) += 1.);
        frequencies
            .values_mut()
            .for_each(|v| *v /= items.len() as f64);
        Self {
            possibilities,
            relations,
            frequencies,
        }
    }

    /// Collapses the wave function at the given location
    pub fn collapse_at(&mut self, l: &Loc) {
        // select choice for possibility
        let mut rng = thread_rng();
        let choice = self.possibilities[l]
            .iter()
            .map(|choice| (*choice, self.frequencies[choice]))
            .collect::<Vec<_>>()
            .choose_weighted(&mut rng, |item| item.1)
            .unwrap()
            .0;
        let item = self.possibilities.get_mut(l).unwrap();
        item.clear();
        item.insert(choice);
    }

    /// Propogates the effects starting from a given location.
    /// Mutates the state of the WFC.
    pub fn propogate(&mut self, start: &Loc) -> Result<(), Loc> {
        let mut changed = vec![*start];
        while let Some(l) = changed.pop() {
            // get all effects from these set of choices
            let mut effects: HashMap<Loc, HashSet<T>> = HashMap::new();
            // must take the & over possibilities but | with relations
            self.possibilities[&l].iter().for_each(|poss| {
                self.relations.get(&poss).map(|rels| {
                    rels.iter().for_each(|rel| {
                        rel.related(&l).drain().for_each(|(l, allowed)| {
                         effects.entry(l)
                          .or_insert_with(HashSet::new)
                          .extend(allowed);
                        })
                    })
                });
            });

            for (loc, permitted) in effects {
                match self.possibilities.get_mut(&loc) {
                    // To easily ignore edges
                    None => continue,
                    Some(prev_posses) => {
                        let inter: HashSet<T> =
                            prev_posses.intersection(&permitted).copied().collect();
                        if inter.is_empty() {
                            return Err(loc);
                        // can just check length because if they differ they will have different
                        // contents
                        } else if inter.len() != prev_posses.len() {
                            changed.push(loc)
                        }
                        *prev_posses = inter;
                    }
                }
            }
        }
        Ok(())
    }
    pub fn shannon_entropy_at(&self, l: &Loc) -> Option<f64> {
        self.possibilities
            .get(l)
            .filter(|posses| posses.len() > 0)
            .map(|possibilities| {
                -possibilities
                    .iter()
                    .map(|poss| self.frequencies[poss])
                    .map(|p| p * p.ln())
                    .sum::<f64>()
            })
    }
    /// returns the index into the location set of what should be used for the next
    /// collapsed location.
    fn next_collapse(&self) -> &Loc {
        self.possibilities
            .keys()
            .filter(|l| self.possibilities[l].len() > 1)
            .map(|l| (l, self.shannon_entropy_at(l)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
    }

    /// collapses the location with the lowest shannon entropy
    /// returns Ok if success or Err or if reached invalid state
    pub fn observe(&mut self) -> Result<(), Loc> {
        let loc = *self.next_collapse();
        self.collapse_at(&loc);
        self.propogate(&loc)
    }

    pub fn is_fully_collapsed(&self) -> bool {
        self.possibilities.values().all(|posses| posses.len() == 1)
    }
    // Returns a hashmap of locations to items or Err if not complete.
    pub fn get_collapsed(&self) -> Result<HashMap<Loc, T>, ()> {
        if !self.is_fully_collapsed() {
            return Err(());
        }
        let ok = self
            .possibilities
            .iter()
            .map(|(&l, choice_of_one)| (l, *choice_of_one.iter().next().unwrap()))
            .collect();
        Ok(ok)
    }

    pub fn get_partial(&self) -> HashMap<Loc, Option<T>> {
        self.possibilities
            .iter()
            .map(|(&l, choices)| {
                (
                    l,
                    if choices.len() == 1 {
                        Some(*choices.iter().next().unwrap())
                    } else {
                        None
                    },
                )
            })
            .collect()
    }

}
