use crate::{
  rels::Dir2D,
  symmetry::{Rot, DEG_0, DEG_180},
};
use std::{
  collections::{HashMap, HashSet},
  hash::Hash,
};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TileDesc<T: Eq + Hash> {
  // How many symmetries are there?
  pub cardinality: usize,
  // Side type -> Rotations
  pub desc: HashMap<T, Vec<Rot>>,
}

impl<T> TileDesc<T>
where
  T: Eq + Hash,
{
  pub fn new(cardinality: usize, desc: impl IntoIterator<Item = (T, Vec<Rot>)>) -> Self
  where
    T: Hash + Eq, {
    Self {
      cardinality,
      desc: desc.into_iter().collect(),
    }
  }
}

pub struct Desc<K: Hash + Copy + Eq, T: Hash + Copy + Eq> {
  // tile name -> tile description
  pub tiles: HashMap<K, TileDesc<T>>,
}

impl<K, T> Desc<K, T>
where
  K: Hash + Copy + Eq,
  T: Hash + Copy + Eq,
{
  /// returns all possible tile configurations for this description
  pub fn items(&self) -> impl Iterator<Item = (K, Rot)> + '_ {
    self
      .tiles
      .iter()
      .flat_map(|(&name, desc)| Rot::up_to(desc.cardinality).map(move |r| (name, r)))
  }
  pub fn rels(&self) -> HashMap<(K, Rot), HashSet<((K, Rot), Dir2D)>> {
    let mut out = HashMap::new();

    self.tiles.iter().for_each(|(&a, desc)| {
      let a_card = desc.cardinality;
      let a_desc = &desc.desc;
      self.tiles.iter().for_each(|(&b, desc)| {
        let b_card = desc.cardinality;
        let b_desc = &desc.desc;

        a_desc.iter().for_each(|(a_side, a_rots)| {
          if let Some(b_rots) = b_desc.get(a_side) {
            a_rots.iter().copied().for_each(|a_rot| {
              b_rots.iter().copied().for_each(|b_rot| {
                // align a to face right
                let a_rot_dest = a_rot.to(DEG_0, a_card);
                // align to face left
                let b_rot_dest = b_rot.to(DEG_180, b_card);
                use Dir2D::*;
                [Right, Up, Left, Down]
                  .iter()
                  .copied()
                  .enumerate()
                  .for_each(|(i, dir)| {
                    let a_side = (a, a_rot_dest.rot_90_n(a_card, i));
                    let b_side = (b, b_rot_dest.rot_90_n(b_card, i));
                    out
                      .entry(a_side)
                      .or_insert_with(HashSet::new)
                      .insert((b_side, dir));
                    out
                      .entry(b_side)
                      .or_insert_with(HashSet::new)
                      .insert((a_side, dir.opp()));
                  })
              });
            })
          }
        });
      });
    });
    out
  }
}
