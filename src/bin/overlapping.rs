#![allow(dead_code, unused_imports)]
extern crate image;
extern crate rand;

use image::{open, DynamicImage, GenericImage, GenericImageView, SubImage};
use std::collections::{HashMap, HashSet};
use wfc::{
    symmetry::{Rot, DEG_0, DEG_180, DEG_270, DEG_90},
    util::generate_2d_positions,
    WaveFunctionCollapse,
};

fn main() {
    let img_path = "./samples/lake.png";
    let img = open(img_path).expect("Failed to open image");
    let imgs = [
        (img.clone().rotate90(), DEG_90),
        (img.clone().rotate180(), DEG_180),
        (img.clone().rotate270(), DEG_270),
        (img, DEG_0),
    ];

    // raw pixel outputs
    let (w, h) = (100, 100);
    let locs = generate_2d_positions(w as usize, h as usize);
    let n = 3u32;
    let mut tiles: HashMap<_, SubImage<_>> = HashMap::new();

    imgs.iter().for_each(|(img, rot)| {
        (0..img.width() - n)
            .flat_map(|x| (0..img.height() - n).map(move |y| (x, y)))
            .for_each(|(x, y)| {
                let sub = img.view(x, y, n, n);
                let ident = tiles.iter().find(|(_, p_sub)| {
                    (0..n)
                        .flat_map(|dx| (0..n).map(move |dy| (dx, dy)))
                        .all(|(dx, dy)| sub.get_pixel(dx, dy) == p_sub.get_pixel(dx, dy))
                });
                if let Some(_) = ident {
                    // TODO add weighting here
                    return;
                }
                tiles.insert((x, y, *rot), sub);
            });
    });
    let aligns = |l: &SubImage<_>, r: &SubImage<_>, dx: i32, dy: i32| {
        let x_min = dx.max(0);
        let x_max = (n as i32).min(dx + n as i32);

        let y_min = dy.max(0);
        let y_max = (n as i32).min(dy + n as i32);

        // TODO should swap order of iteration for speed?
        (x_min..x_max).all(|x| {
            (y_min..y_max).all(|y| {
                l.get_pixel(x as u32, y as u32) == r.get_pixel((x - dx) as u32, (y - dy) as u32)
            })
        })
    };

    let mut rels = HashMap::new();
    let dirs = (-1..=1)
        .flat_map(move |x| (-1..=1).map(move |y| (x, y)))
        .filter(|&coord| coord != (0, 0));

    // slide over with 2n-1 in both directions to see valid configurations.
    tiles.iter().for_each(|(&a_loc, a)| {
        tiles.iter().for_each(|(&b_loc, b)| {
            dirs.clone().for_each(|diff| {
                if aligns(a, b, diff.0, diff.1) {
                    rels.entry(a_loc)
                        .or_insert_with(HashSet::new)
                        .insert((b_loc, diff));
                }
            });
        });
    });

    let items: Vec<(u32, u32, Rot)> = rels.keys().copied().collect();
    let mut waves = WaveFunctionCollapse::new(locs, items, rels);
    while !waves.is_fully_collapsed() {
        match waves.observe() {
            Ok(()) => (),
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    let mut out = DynamicImage::new_rgba8(w + n, h + n);
    waves
        .get_partial()
        .drain()
        .filter_map(|(k, v)| v.map(|v| (k, v)))
        .for_each(|((x, y), pos)| {
            out.copy_from(&tiles[&pos], x as u32, y as u32);
        });
    out.save(format!("ol_{}_{}.png", w, h))
        .expect("failed to save output");
}
