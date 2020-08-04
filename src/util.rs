pub fn generate_2d_positions(width: u32, height: u32) -> impl Iterator<Item = (usize, usize)> {
  (0..height).flat_map(move |y| (0..width).map(move |x| (x as usize, y as usize)))
}
