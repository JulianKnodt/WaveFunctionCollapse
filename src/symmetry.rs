/// Rotations in degrees
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rot(usize);
// Right
pub const DEG_0: Rot = Rot(0);
// Up
pub const DEG_90: Rot = Rot(90);
// Left
pub const DEG_180: Rot = Rot(180);
// Down
pub const DEG_270: Rot = Rot(270);

impl Rot {
    pub const fn rot_90(&self, card: usize) -> Self {
        Rot((self.0 + 90) % card)
    }
    pub const fn opp(&self) -> Self {
        Rot((self.0 + 180) % 360)
    }
    pub fn to(&self, o: &Self, card: usize) -> Self {
        let dest = if o.0 < self.0 { o.0 + 360 } else { o.0 };
        Rot((dest - self.0) % card)
    }
    pub const fn rot_90_n(&self, card: usize, n: usize) -> Self {
        Rot((self.0 + 90 * n) % card)
    }
    pub fn up_to(v: usize) -> Vec<Rot> {
        (0..v).step_by(90).map(|deg| Rot(deg)).collect()
    }
}

#[test]
fn to() {
    assert_eq!(DEG_0.to(&DEG_90, 360), DEG_90);
    assert_eq!(DEG_0.to(&DEG_180, 360), DEG_180);
    assert_eq!(DEG_0.to(&DEG_270, 360), DEG_270);
    assert_eq!(DEG_0.to(&DEG_270, 90), DEG_0);
    assert_eq!(DEG_270.to(&DEG_0, 360), DEG_90);
    assert_eq!(DEG_180.to(&DEG_0, 360), DEG_180);
    assert_eq!(DEG_180.to(&DEG_0, 180), DEG_0);
    assert_eq!(DEG_90.to(&DEG_0, 180), DEG_90);
}

#[test]
fn rot_90_n() {
    assert_eq!(DEG_0.rot_90_n(360, 0), DEG_0);
    assert_eq!(DEG_0.rot_90_n(360, 1), DEG_90);
}
