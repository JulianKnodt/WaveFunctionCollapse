extern crate image;
extern crate rand;

mod wfc;
pub use wfc::WaveFunctionCollapse;
pub mod rels;

pub mod symmetry;

pub mod util;

pub mod tiles;

mod instance;
pub use instance::Instance;
