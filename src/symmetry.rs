/// Rotations in degrees
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rot(usize);
pub const DEG_0: Rot = Rot(0);
pub const DEG_90: Rot = Rot(90);
pub const DEG_180: Rot = Rot(180);
pub const DEG_270: Rot = Rot(270);

impl Rot {
    pub const fn ident(&self, _card: usize) -> Self {
        *self
    }
    pub const fn opposite(&self, card: usize) -> Rot {
        Rot((self.0 + 180) % card)
    }
    pub const fn rot_90(&self, card: usize) -> Rot {
        Rot((self.0 + 90) % card)
    }
    pub const fn rot_neg_90(&self, card: usize) -> Rot {
        Rot((self.0 + 270) % card)
    }
    pub const fn across_x(&self, card: usize) -> Rot {
        Rot((360 - self.0) % card)
    }
    pub fn across_minor_diag(&self, card: usize) -> Rot {
        if self.0 % 180 == 0 {
            self.rot_90(card)
        } else {
            self.rot_neg_90(card)
        }
    }
    pub fn across_y(&self, card: usize) -> Rot {
        if self.0 % 180 == 0 {
            *self
        } else {
            self.opposite(card)
        }
    }
    pub fn across_major_diag(&self, card: usize) -> Rot {
        if self.0 % 180 == 0 {
            self.rot_neg_90(card)
        } else {
            self.rot_90(card)
        }
    }
    /// allowed rotations under a given cardinality
    pub fn equal_under(&self, s: Symmetry) -> Vec<Rot> {
        use Symmetry::*;
        let c = s.cardinality();
        match s {
            // None other than original are equal
            Uniq => vec![self.ident(c)],
            X => vec![self.ident(c), self.across_x(c)],
            Y => vec![self.ident(c), self.across_y(c)],
            XY => vec![self.ident(c), self.across_x(c), self.across_y(c)],

            MajorDiag => vec![self.ident(c), self.across_major_diag(c)],
            MinorDiag => vec![self.ident(c), self.across_minor_diag(c)],
            BothDiags => vec![
                self.ident(c),
                self.across_major_diag(c),
                self.across_minor_diag(c),
            ],
        }
    }
}

/// Various kinds of symmetries that are permitted for 2d tiles
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Symmetry {
    /// Both X and Y
    XY,
    /// Reflection across X axis
    X,
    /// Reflection across Y axis
    Y,
    /// Diagonal from Upper Left to Lower Right
    MajorDiag,
    /// Diagonal from lower left to upper right
    MinorDiag,
    /// Both Diagonals
    BothDiags,
    /// No special Symmetry
    Uniq,
}

impl Symmetry {
    /// maximum degrees before it returns to original.
    pub fn cardinality(&self) -> usize {
        use Symmetry::*;
        match self {
            XY => 90,
            X | Y => 180,
            MajorDiag | MinorDiag | BothDiags => 180,
            Uniq => 360,
        }
    }
}

pub struct Openings {
  top: bool,
  bot: bool,
  left: bool,
  right: bool,
}

// TODO implement Openings struct "cardinality"
// and allowed rotations for each.

use crate::rels::Dir2D;
use crate::wfc::Item;
use std::collections::{HashMap, HashSet};
/// Gets all 2D relations between left and right which would be equivalent
pub fn symmetry_relations<T: Item>(
    l: T,
    l_rot: Rot,
    r: T,
    r_rot: Rot,
    l_sym: Symmetry,
    r_sym: Symmetry,
) -> HashMap<(T, Rot), HashSet<((T, Rot), Dir2D)>> {
    let mut out = HashMap::new();
    // currently we have the l-r relationship, now need to derive the rest
    let l_allowed = l_rot.equal_under(l_sym);
    let r_allowed = r_rot.equal_under(r_sym);
    let mut add = |ls: &Vec<Rot>, rs: &Vec<Rot>, dir| {
        ls.iter().for_each(|&ls_rot| {
            rs.iter().for_each(|&rs_rot| {
                out.entry((l, ls_rot))
                    .or_insert_with(|| HashSet::new())
                    .insert(((r, rs_rot), dir));
            });
        })
    };

    let l_card = l_sym.cardinality();
    let r_card = r_sym.cardinality();
    // actually add all rotations
    add(&l_allowed, &r_allowed, Dir2D::Right);
    add(
        &l_allowed.iter().map(|&r| r.rot_90(l_card)).collect(),
        &r_allowed.iter().map(|&r| r.rot_90(r_card)).collect(),
        Dir2D::Up,
    );
    add(
        &l_allowed.iter().map(|&r| r.opposite(l_card)).collect(),
        &r_allowed.iter().map(|&r| r.opposite(r_card)).collect(),
        Dir2D::Left,
    );
    add(
        &l_allowed.iter().map(|&r| r.rot_neg_90(l_card)).collect(),
        &r_allowed.iter().map(|&r| r.rot_neg_90(r_card)).collect(),
        Dir2D::Down,
    );
    out
}

