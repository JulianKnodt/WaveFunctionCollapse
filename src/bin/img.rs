#![allow(dead_code, unused_imports)]
extern crate cpuprofiler;
extern crate image;
extern crate rand;

use image::{open, DynamicImage, GenericImage};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use wfc::{
    rels::Dir2D,
    symmetry::{Rot, DEG_0, DEG_180, DEG_270, DEG_90},
    tiles::{Desc, TileDesc},
    util::generate_2d_positions,
    WaveFunctionCollapse,
};

fn main() {
    cpuprofiler::PROFILER
        .lock()
        .unwrap()
        .start("./img.profile")
        .expect("failed to start");
    run();
    cpuprofiler::PROFILER
        .lock()
        .unwrap()
        .stop()
        .expect("failed to stop");
}

fn run() {
    let desc = circuit_description();
    let folder = "circuits";
    let tile_size = 14;
    let (w, h) = (100, 50);

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
    let mut out = DynamicImage::new_rgba8((w * tile_size) as u32, (h * tile_size) as u32);
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
                let img = open(format!("./samples/{}/{}", folder, img)).expect("failed to open");
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
            out.copy_from(ent, (x * tile_size) as u32, (y * tile_size) as u32);
        });
    out.save(format!("{}_{}_{}.png", folder, w, h))
        .expect("failed to save output");
}

#[allow(unused_variables)]
fn knot_description() -> Desc<&'static str, bool> {
    let cross_tile_desc = TileDesc::from((180, vec![(true, vec![DEG_90, DEG_0])]));
    let corner_tile_desc = TileDesc::from((
        360,
        vec![(true, vec![DEG_0, DEG_270]), (false, vec![DEG_180, DEG_90])],
    ));
    let empty_tile_desc = TileDesc::from((90, vec![(false, vec![DEG_0])]));
    let t_tile_desc = TileDesc::from((
        360,
        vec![(true, vec![DEG_0, DEG_180, DEG_90]), (false, vec![DEG_270])],
    ));
    let line_tile_desc = TileDesc::from((180, vec![(true, vec![DEG_0]), (false, vec![DEG_90])]));
    Desc {
        tiles: [
            ("corner.png", corner_tile_desc),
            ("t.png", t_tile_desc),
            ("empty.png", empty_tile_desc),
            ("cross.png", cross_tile_desc),
            ("line.png", line_tile_desc),
        ]
        .into_iter()
        .cloned()
        .collect(),
    }
}

#[allow(unused_variables)]
fn circuit_description() -> Desc<&'static str, usize> {
    let bridge = TileDesc::from((180, vec![(0, vec![DEG_0]), (1, vec![DEG_90])]));
    let component = TileDesc::from((90, vec![(2, vec![DEG_0])]));
    let connection = TileDesc::from((
        360,
        vec![
            (3, vec![DEG_0]),
            (2, vec![DEG_90]),
            (4, vec![DEG_180]),
            (1, vec![DEG_270]),
        ],
    ));
    let corner = TileDesc::from((
        360,
        vec![
            (5, vec![DEG_0, DEG_270]),
            (4, vec![DEG_180]),
            (3, vec![DEG_90]),
        ],
    ));
    let dskew = TileDesc::from((180, vec![(1, vec![DEG_0, DEG_90])]));
    let skew = TileDesc::from((
        360,
        vec![(1, vec![DEG_0, DEG_270]), (5, vec![DEG_90, DEG_180])],
    ));
    let substrate = TileDesc::from((90, vec![(5, vec![DEG_0])]));
    let t = TileDesc::from((
        360,
        vec![(1, vec![DEG_0, DEG_90, DEG_180]), (5, vec![DEG_270])],
    ));
    let track = TileDesc::from((180, vec![(5, vec![DEG_0]), (1, vec![DEG_90])]));
    let transition = TileDesc::from((
        360,
        vec![
            (5, vec![DEG_0, DEG_180]),
            (1, vec![DEG_90]),
            (0, vec![DEG_270]),
        ],
    ));
    let turn = TileDesc::from((
        360,
        vec![(1, vec![DEG_0, DEG_270]), (5, vec![DEG_90, DEG_180])],
    ));
    let viad = TileDesc::from((180, vec![(1, vec![DEG_0]), (5, vec![DEG_90])]));
    let vias = TileDesc::from((
        360,
        vec![(1, vec![DEG_270]), (5, vec![DEG_0, DEG_90, DEG_180])],
    ));
    let wire = TileDesc::from((180, vec![(0, vec![DEG_0]), (5, vec![DEG_90])]));
    Desc {
        tiles: [
            ("bridge.png", bridge),
            // -- don't work due to asymmetric rotation, need to fix this
            // ("component.png", component),
            // ("connection.png", connection),
            // ("corner.png", corner),
            ("dskew.png", dskew),
            ("skew.png", skew),
            ("substrate.png", substrate),
            ("t.png", t),
            ("track.png", track),
            ("transition.png", transition),
            ("turn.png", turn),
            ("viad.png", viad),
            ("vias.png", vias),
            ("wire.png", wire),
        ]
        .into_iter()
        .cloned()
        .collect(),
    }
}
