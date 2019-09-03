#![allow(dead_code, unused_imports)]
extern crate image;

use image::{open, DynamicImage, GenericImage};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use wfc::{
    rels::Dir2D,
    symmetry::{Rot, DEG_0, DEG_180, DEG_270, DEG_90},
    util::{flatten, generate_2d_positions},
    WaveFunctionCollapse,
};

fn main() {
    let desc = knot_description();
    let (w, h) = (50, 50);
    let locs = generate_2d_positions(w, h);
    let items = desc.items();
    let rels = desc.rels();
    let mut waves = WaveFunctionCollapse::new(locs, items, rels);
    while !waves.is_fully_collapsed() {
        match waves.observe() {
            Ok(()) => (),
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    let mut out = DynamicImage::new_rgba8((w * desc.tile_size) as u32, (h * desc.tile_size) as u32);
    let mut img_cache = HashMap::new();
    waves
        .get_partial()
        .drain()
        .filter_map(|(k, v)| match v {
            None => None,
            Some(v) => Some((k, v)),
        })
        .for_each(|((x, y), (img, rot))| {
            let ent = img_cache.entry((img, rot)).or_insert_with(|| {
                let img =
                    open(format!("./samples/{}/{}", desc.folder, img)).expect("failed to open");
                if rot == DEG_0 {
                    img
                } else if rot == DEG_90 {
                    img.rotate90()
                } else if rot == DEG_180 {
                    img.rotate180()
                } else if rot == DEG_270 {
                    img.rotate270()
                } else {
                    unreachable!()
                }
            });
            out.copy_from(
                ent,
                (x * desc.tile_size) as u32,
                (y * desc.tile_size) as u32,
            );
        });
    out.save(format!("{}_{}_{}.png", desc.folder, w, h))
        .expect("failed to save output");
}

#[derive(Clone, Debug)]
pub struct TileDesc<T: Hash + Copy + Eq> {
    cardinality: usize,
    desc: HashMap<T, Vec<Rot>>,
}

pub struct Desc<T: Hash + Copy + Eq> {
    folder: &'static str,
    // all tiles are expected to be tile_size x tile_size
    tile_size: usize,
    // tile name -> tile description
    tiles: HashMap<&'static str, TileDesc<T>>,
}

impl<T> Desc<T>
where
    T: Hash + Copy + Eq,
{
    /// returns all possible tile configurations for this description
    pub fn items(&self) -> Vec<(&'static str, Rot)> {
        self.tiles
            .iter()
            .flat_map(|(&name, desc)| {
                Rot::up_to(desc.cardinality)
                    .into_iter()
                    .map(move |r| (name, r))
            })
            .collect()
    }
    pub fn rels(&self) -> HashMap<(&'static str, Rot), HashSet<((&'static str, Rot), Dir2D)>> {
        let mut out = HashMap::new();

        self.tiles.iter().for_each(|(&a, desc)| {
            let a_card = desc.cardinality;
            let a_desc = &desc.desc;
            self.tiles.iter().for_each(|(&b, desc)| {
                let b_card = desc.cardinality;
                let b_desc = &desc.desc;

                a_desc.iter().for_each(|(a_side, a_rots)| {
                    b_desc.get(a_side).map(|b_rots| {
                        a_rots.iter().copied().for_each(|a_rot| {
                            b_rots.iter().copied().for_each(|b_rot| {
                                // align a to face right
                                let a_rot_dest = a_rot.to(&DEG_0, a_card);
                                // align to face left
                                let b_rot_dest = b_rot.to(&DEG_180, b_card);
                                use Dir2D::*;
                                [Right, Up, Left, Down]
                                    .into_iter()
                                    .copied()
                                    .enumerate()
                                    .for_each(|(i, dir)| {
                                        let a_side = (a, a_rot_dest.rot_90_n(a_card, i));
                                        let b_side = (b, b_rot_dest.rot_90_n(b_card, i));
                                        out.entry(a_side)
                                            .or_insert_with(HashSet::new)
                                            .insert((b_side, dir));
                                        out.entry(b_side)
                                            .or_insert_with(HashSet::new)
                                            .insert((a_side, dir.opp()));
                                    })
                            });
                        })
                    });
                });
            });
        });
        out
    }
}

fn knot_description() -> Desc<bool> {
    let cross = "cross.png";
    let cross_tile_desc = TileDesc {
        cardinality: 180,
        desc: [(true, vec![DEG_90, DEG_0])].iter().cloned().collect(),
    };
    let corner = "corner.png";
    let corner_tile_desc = TileDesc {
        cardinality: 360,
        desc: [(true, vec![DEG_0, DEG_270]), (false, vec![DEG_180, DEG_90])]
            .iter()
            .cloned()
            .collect(),
    };
    let empty = "empty.png";
    let empty_tile_desc = TileDesc {
        cardinality: 90,
        desc: [(false, vec![DEG_0])].iter().cloned().collect(),
    };
    let t = "t.png";
    let t_tile_desc = TileDesc {
        cardinality: 360,
        desc: [(true, vec![DEG_0, DEG_180, DEG_90]), (false, vec![DEG_270])]
            .iter()
            .cloned()
            .collect(),
    };
    let line = "line.png";
    let line_tile_desc = TileDesc {
        cardinality: 180,
        desc: [(true, vec![DEG_0]), (false, vec![DEG_90])]
            .iter()
            .cloned()
            .collect(),
    };
    Desc {
        folder: "knots",
        tile_size: 10,
        tiles: [
            (corner, corner_tile_desc),
            (t, t_tile_desc),
            (empty, empty_tile_desc),
            (cross, cross_tile_desc),
            (line, line_tile_desc),
        ]
        .into_iter()
        .cloned()
        .collect(),
    }
}
