#![allow(dead_code, unused_imports)]

use image::{DynamicImage, GenericImage};
use std::{
  collections::HashMap,
  fs::canonicalize,
  path::Path,
  time::{Duration, Instant},
};

use wfc::{
  rels::Dir2D,
  symmetry::{Rot, DEG_0, DEG_180, DEG_270, DEG_90},
  tiles::{Desc, TileDesc},
  util::generate_2d_positions,
  Instance, WaveFunctionCollapse,
};

fn main() {
  let desc = Instance::knot_instance();
  let folder = "knot";
  let (w, h) = (50, 50);
  let tile_size = desc.tile_size as usize;

  let locs = generate_2d_positions(w as u32, h as u32);
  let items = desc.items();
  let rels = desc.rels();
  let mut waves = WaveFunctionCollapse::new(locs, items, rels);
  while !waves.is_fully_collapsed() {
    assert!(waves.observe().is_ok());
  }

  let mut out = DynamicImage::new_rgba8((w * tile_size) as u32, (h * tile_size) as u32);
  let mut img_cache = HashMap::new();
  for ((x, y), v) in waves.get_partial() {
    let (img, rot) = if let Some(v) = v { v } else { continue };

    let ent = img_cache.entry((img, rot)).or_insert_with(|| {
      let img = image::open(Path::new(&desc.image_dir).join(img)).expect("failed to open");
      match rot {
        DEG_0 => img,
        DEG_90 => img.rotate90(),
        DEG_180 => img.rotate180(),
        DEG_270 => img.rotate270(),
        _ => unreachable!(),
      }
    });

    out.copy_from(ent, (x * tile_size) as u32, (y * tile_size) as u32);
  }

  let dir = format!("{}_{}_{}.png", folder, w, h);
  out.save(&dir).expect("failed to save output");
  println!("Saved to {}", dir);
}
