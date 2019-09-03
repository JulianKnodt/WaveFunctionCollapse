use wfc::{
    rels::get_2d_rels,
    util::{flatten, generate_2d_positions},
    WaveFunctionCollapse,
};

fn main() {
    let (width, height) = (50, 10);
    let locs = generate_2d_positions(width, height);
    let example = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 1, 1, 0],
        vec![1, 2, 2, 1],
        vec![2, 2, 2, 2],
        vec![2, 2, 2, 2],
    ];
    let relations = get_2d_rels(&example);
    let items = example.iter().flatten().copied().collect();
    let mut wfc = WaveFunctionCollapse::new(locs, items, relations);
    while !wfc.is_fully_collapsed() {
        match wfc.observe() {
            Ok(()) => (),
            Err(e) => {
                println!("{:?}", e);
                return;
            }
        }
    }
    let flat = flatten(wfc.get_collapsed().unwrap());
    (0..height).for_each(|y| {
        (0..width).for_each(|x| {
            print!("{} ", flat[x * height + y]);
        });
        println!();
    });
}
