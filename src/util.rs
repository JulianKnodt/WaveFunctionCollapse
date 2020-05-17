use std::collections::HashMap;

pub fn generate_2d_positions(width: usize, height: usize) -> Vec<(usize, usize)> {
  (0..height)
    .flat_map(|y| (0..width).map(move |x| (x, y)))
    .collect()
}

pub fn flatten<O: Ord + Copy, T>(mut from: HashMap<O, T>) -> Vec<T> {
  let mut out = from.drain().collect::<Vec<_>>();
  out.sort_unstable_by_key(|v| v.0);
  out.into_iter().map(|v| v.1).collect()
}
