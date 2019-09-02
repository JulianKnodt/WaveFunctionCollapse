#![allow(dead_code, unused_imports)]
extern crate image;

use std::collections::{HashMap, HashSet};
// use image::{open, GenericImageView};
use std::ops::Hash;

use wfc::{
    symmetry::{Rot, Symmetry, DEG_0, DEG_180, DEG_270, DEG_90},
    WaveFunctionCollapse,
};

fn main() {
    let desc = knot_description();
}

pub struct Desc<T: Hash + Copy + Eq> {
    name: &'static str,
    tiles: HashMap<&'static str, (usize, HashMap<T, Rot>)>,
}

fn knot_description() -> Desc {
    let cross = "cross";
    let corner = "corner";
    let empty = "empty";
    let t = "t";
    let line = "line";
    Desc {
        name: "knot",
        tiles: [
            (corner, ),
            (t, Symmetry::XY),
            (empty, Symmetry::Y),
            (cross, Symmetry::XY),
            (line, Symmetry::X),
        ]
        .into_iter()
        .copied()
        .collect()
    }
}
