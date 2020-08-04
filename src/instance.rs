use crate::{
  rels::{Dir2D, Neighbor2D},
  symmetry::*,
  tiles::TileDesc,
};
use std::{collections::HashMap, hash::Hash, path::Path};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Instance<T: Hash + Eq> {
  pub image_dir: String,
  pub tile_size: u32,

  // Image file name -> tile_description
  description: HashMap<String, TileDesc<T>>,
}

pub type RotImg<'a> = (&'a str, Rot);

impl<T: Hash + Eq> Instance<T> {
  pub fn items(&self) -> impl Iterator<Item = RotImg<'_>> + '_ {
    self
      .description
      .iter()
      .flat_map(|(name, desc)| Rot::up_to(desc.cardinality).map(move |r| (name.as_str(), r)))
  }
  pub fn rels(&self) -> HashMap<RotImg<'_>, Vec<Neighbor2D<RotImg<'_>>>> {
    // TODO there is a bug here somewhere
    let mut out = HashMap::new();

    for (a, src) in self.description.iter() {
      for (b, dst) in self.description.iter() {
        for (a_side, a_rots) in src.desc.iter() {
          // Does this have a matching side?
          let b_rots = if let Some(b_rots) = dst.desc.get(a_side) {
            b_rots
          } else {
            continue;
          };
          // if so add all ways to match up the two sides
          for a_rot in a_rots.iter() {
            for b_rot in b_rots.iter() {
              let a_rot_dest = a_rot.to(DEG_0, src.cardinality);
              let b_rot_dest = b_rot.to(DEG_180, dst.cardinality);
              use Dir2D::*;
              for (i, &dir) in [Right, Up, Left, Down].iter().enumerate() {
                let a_side = (a.as_str(), a_rot_dest.rot_90_n(src.cardinality, i));
                let b_side = (b.as_str(), b_rot_dest.rot_90_n(dst.cardinality, i));
                out
                  .entry(a_side)
                  .or_insert_with(Vec::new)
                  .push(Neighbor2D::new(b_side, dir));
                out
                  .entry(b_side)
                  .or_insert_with(Vec::new)
                  .push(Neighbor2D::new(a_side, dir.opp()));
              }
            }
          }
        }
      }
    }
    for v in out.values_mut() {
      v.sort_unstable();
      v.dedup();
    }
    out
  }
}

macro_rules! tile_desc {
  ($name: expr, $cardinality: expr, [$(($edge: expr, [$($sym: expr$(,)?)+]) $(,)? )+]) => {{
    (
      String::from($name),
      TileDesc::new($cardinality,
        vec![ $((
          $edge, vec![$( $sym, )+],
        ),)+]
      )
    )
  }}
}

impl Instance<bool> {
  pub fn knot_instance() -> Self {
    Self {
      image_dir: Path::new("./")
        .join(file!())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("bin/samples/knots")
        .to_string_lossy()
        .into_owned(),

      tile_size: 10,
      description: vec![
        tile_desc!("cross.png", 180, [(true, [DEG_90, DEG_0])]),
        tile_desc!("corner.png", 360, [
          (true, [DEG_0, DEG_270]),
          (false, [DEG_180, DEG_90])
        ]),
        tile_desc!("empty.png", 90, [(false, [DEG_0])]),
        tile_desc!("t.png", 360, [
          (true, [DEG_0, DEG_90, DEG_180]),
          (false, [DEG_270])
        ]),
        tile_desc!("line.png", 180, [(true, [DEG_0]), (false, [DEG_90])]),
      ]
      .into_iter()
      .collect(),
    }
  }
}

impl Instance<usize> {
  /*
  TODO
  pub fn circuit_description() -> Self {
    Instance {
      image_dir: String::from("./samples/circuits"),
      description: vec![
        (
          "bridge",
          TileDesc::new(180, vec![(0, vec![DEG_0]), (1, vec![DEG_90])]),
        ),
        (
          "component", TileDesc::new(90, vec![(2, vec![DEG_0])]),
        ),
        (
          "connection", TileDesc::new(360, vec![(1, vec![DEG_0])]),
        ),
      ]
      .into_iter()
      .map(|(k, v)| (String::from(k), v))
      .collect(),
      tiles: [
        ("bridge.png", "bridge"),
        ("cross.png", "cross"),
        ("empty.png", "empty"),
        ("line.png", "line"),
        ("t.png", "t"),
      ]
      .iter()
      .map(|(k, v)| (String::from(*k), String::from(*v)))
      .collect(),
    }
  }
  */
}
