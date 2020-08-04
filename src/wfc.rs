use rand::{prelude::*, seq::SliceRandom};
use std::{collections::HashMap, fmt::Debug, hash::Hash};

/// Represents the relation between a location and neighbors
pub trait Relation: Debug {
  type Loc;
  type Item;

  /// Returns an iterator over all relations to a given location
  fn related(&self, at: Self::Loc, f: impl FnMut(Self::Loc, Self::Item));
}

/// WaveFunctionCollapse maintains state for the wave function collapse function
#[derive(Debug)]
pub struct WaveFunctionCollapse<L, T, R> {
  pub possibilities: HashMap<L, Vec<T>>,
  pub relations: HashMap<T, Vec<R>>,
  frequencies: HashMap<T, f32>,

  entropy_cache: HashMap<L, f32>,
}

impl<L, T, R> WaveFunctionCollapse<L, T, R>
where
  L: Hash + Eq + Copy + Debug + Ord,
  T: Hash + Eq + Copy + Debug,
  R: Relation<Loc = L, Item = T>,
{
  pub fn new(
    locations: impl IntoIterator<Item = L>,
    items: impl IntoIterator<Item = T>,
    relations: impl IntoIterator<Item = (T, Vec<R>)>,
  ) -> Self {
    let rels: HashMap<_, _> = relations.into_iter().collect();
    let uniq_items: Vec<T> = items.into_iter().collect();

    let possibilities: HashMap<_, _> = locations
      .into_iter()
      .map(|l| (l, uniq_items.clone()))
      .collect();

    let freqs: HashMap<_, _> = rels
      .keys()
      .map(|v| (*v, (possibilities.len() as f32).recip()))
      .collect();

    let mut out = Self {
      possibilities,
      relations: rels,
      frequencies: freqs,

      entropy_cache: HashMap::new(),
    };

    for l in out.possibilities.keys() {
      out.entropy_cache.insert(*l, out.shannon_entropy_at(&l));
    }
    out
  }

  /// Collapses the wave function at the given location
  pub fn collapse_at(&mut self, l: L) {
    // select choice for possibility
    let mut rng = thread_rng();
    // can move the buffer to the WFC struct
    let mut buffer = vec![];

    buffer.extend(
      self.possibilities[&l]
        .iter()
        .map(|&choice| (choice, self.frequencies[&choice])),
    );
    assert!(!buffer.is_empty());

    let choice = buffer.choose_weighted(&mut rng, |item| item.1).unwrap().0;
    let item = self.possibilities.get_mut(&l).unwrap();
    item.clear();
    item.push(choice);
    buffer.clear();
  }

  /// Propogates the effects starting from a given location.
  /// Mutates the state of the WFC.
  pub fn propogate(&mut self, start: L) -> Result<(), &'static str> {
    let mut changed = vec![start];
    let mut touched = vec![start];

    // get all effects from these set of choices
    let mut effects: HashMap<L, Vec<T>> = HashMap::new();

    while let Some(l) = changed.pop() {
      // must take the & over possibilities but | with relations
      for poss in self.possibilities[&l].iter() {
        for relation in self.relations[&poss].iter() {
          relation.related(l, |related, allowed| {
            // TODO this is missing the effects
            effects
              .entry(related)
              .or_insert_with(Vec::new)
              .push(allowed);
          });
        }
      }

      // TODO possibly combine these two loops
      for (loc, mut permitted) in effects.drain() {
        if let Some(ref mut prev_posses) = self.possibilities.get_mut(&loc) {
          permitted.dedup();

          let original_len = prev_posses.len();

          prev_posses.retain(|l| permitted.contains(&l));
          if prev_posses.is_empty() {
            return Err("Failed wfc");
          }
          if prev_posses.len() != original_len {
            if !changed.contains(&loc) {
              changed.push(loc);
            }
            touched.push(loc);
          }
        }
      }
    }

    touched.sort_unstable();
    touched.dedup();
    for l in touched.drain(..) {
      self.entropy_cache.insert(l, self.shannon_entropy_at(&l));
    }
    Ok(())
  }

  /// Computers the shannon entropy at a location.
  /// Stateless, so if caching is necessary use the entropy_else_get fn.
  pub fn shannon_entropy_at(&self, l: &L) -> f32 {
    let posses = &self.possibilities[l];
    assert!(!posses.is_empty());
    -posses
      .iter()
      .map(|poss| self.frequencies[poss])
      .map(|p| p * p.ln())
      .sum::<f32>()
  }
  /// returns the index into the location set of what should be used for the next
  /// collapsed location.
  fn next_collapse(&self) -> &L {
    self
      .possibilities
      .iter()
      .filter(|(_, posses)| posses.len() > 1)
      .map(|(l, _)| (l, self.entropy_cache[&l]))
      .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
      .expect("none left")
      .0
  }

  /// collapses the location with the lowest shannon entropy
  /// returns Ok if success or Err or if reached invalid state
  pub fn observe(&mut self) -> Result<(), &'static str> {
    let l = *self.next_collapse();
    self.collapse_at(l);
    self.propogate(l)
  }

  /// returns the number of complete tiles and the total number of tiles
  pub fn num_complete(&self) -> (usize, usize) {
    (
      self
        .possibilities
        .values()
        .filter(|poss| poss.len() == 1)
        .count(),
      self.possibilities.len(),
    )
  }

  pub fn is_fully_collapsed(&self) -> bool {
    self.possibilities.values().all(|posses| posses.len() <= 1)
  }
  // Returns a hashmap of locations to items or Err if not complete.
  pub fn get_collapsed(&self) -> Result<impl Iterator<Item = (L, T)> + '_, ()> {
    if !self.is_fully_collapsed() {
      return Err(());
    }
    let ok = self
      .possibilities
      .iter()
      .map(|(&l, choice_of_one)| (l, choice_of_one[0]));
    Ok(ok)
  }

  pub fn get_partial(&self) -> impl Iterator<Item = (L, Option<T>)> + '_ {
    self.possibilities.iter().map(|(&l, choices)| {
      (
        l,
        if choices.len() == 1 {
          Some(*choices.iter().next().unwrap())
        } else {
          None
        },
      )
    })
  }
}
