use wfc::{grid::Grid, rels::get_2d_rels, WaveFunctionCollapse};

fn main() {
    let n = 50;
    let locs: Vec<(usize, usize)> = (0..n).flat_map(|i| (0..n).map(move |j| (i, j))).collect();
    let example = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 1, 1, 0],
        vec![1, 2, 2, 1],
        vec![2, 2, 2, 2],
        vec![2, 2, 2, 2],
    ];
    let relations = get_2d_rels(&example);
    let mut wfc =
        WaveFunctionCollapse::new(locs, example.iter().flatten().copied().collect(), relations);
    while !wfc.is_fully_collapsed() {
        match wfc.observe() {
            Ok(()) => (),
            Err(e) => {
                println!();
                let g = Grid::new(n, wfc.get_partial());
                g.display_partial();
                println!("{:?}", e);
                return;
            }
        }
    }
    let g = Grid::new(n, wfc.get_collapsed().unwrap());
    g.display();
}
