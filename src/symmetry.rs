// Various kinds of symmetries that are permitted for 2d tiles
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Symmetry {
    // Complete rotation & reflection sym
    Central,
    // Reflection across X axis
    X,
    // Reflection across Y axis
    Y,
    // Diagonal from Upper Left to Lower Right
    MajorDiag,
    // Diagonal from lower left to upper right
    MinorDiag,

    // Rotational around the center
    Rotational,

    // No special Symmetry Properties
    Uniq,
}

pub fn symmetry1d<T: PartialEq>(items: &[T]) -> bool {
    let len = items.len() - 1;
    (0..(len + 1) / 2).all(|x| items[x] == items[len - x])
}

// finds symmetry in a square of items
pub fn symmetry2d<T: PartialEq>(items: &[&[T]]) -> Symmetry {
    let y_sym = items.iter().all(|row| symmetry1d(row));
    let len = items.len() - 1;
    let x_sym = (0..(len + 1) / 2).all(|y| items[y] == items[len - y]);

    match (y_sym, x_sym) {
        (true, true) => return Symmetry::Central,
        (true, false) => return Symmetry::Y,
        (false, true) => return Symmetry::X,
        (false, false) => (),
    };

    // Could be empty, handle this later
    let width = items[0].len() - 1;
    let major_diag_sym =
        (0..=len).all(|y| (0..=width - y).all(|x| items[y][x] == items[width - x][len - y]));
    let minor_diag_sym = (0..=len).all(|y| (y..=width).all(|x| items[y][x] == items[x][y]));

    return match (major_diag_sym, minor_diag_sym) {
        (true, true) => Symmetry::Rotational,
        (true, false) => Symmetry::MajorDiag,
        (false, true) => Symmetry::MinorDiag,
        (false, false) => Symmetry::Uniq,
    };
}

#[test]
fn test1d() {
    let items = [0, 1, 2, 3, 4, 5];
    assert!(!symmetry1d(&items));
    let items = [0, 1, 2, 1, 0];
    assert!(symmetry1d(&items));
    let items = [0, 1, 1, 0];
    assert!(symmetry1d(&items));
}

#[test]
fn test2d() {
    let items = [&[0, 0] as &[i32], &[0, 0]];
    assert_eq!(symmetry2d(&items), Symmetry::Central);
    let items = [&[0, 0] as &[i32], &[1, 1]];
    assert_eq!(symmetry2d(&items), Symmetry::Y);
    let items = [&[1, 0] as &[i32], &[1, 0]];
    assert_eq!(symmetry2d(&items), Symmetry::X);
    let items = [&[1, 0] as &[i32], &[0, 1]];
    assert_eq!(symmetry2d(&items), Symmetry::Rotational);
    let items = [&[1, 0] as &[i32], &[2, 1]];
    assert_eq!(symmetry2d(&items), Symmetry::MajorDiag);
    let items = [&[0, 1] as &[i32], &[1, 2]];
    assert_eq!(symmetry2d(&items), Symmetry::MinorDiag);
    let items = [&[0, 1] as &[i32], &[2, 3]];
    assert_eq!(symmetry2d(&items), Symmetry::Uniq);
}
