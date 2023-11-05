mod macros;
mod window;

use crate::{impl_absorber, impl_body, impl_insulator, impl_reflective};
pub use window::*;

// for all of these:
// enum name, pretty name, cost, ...

impl_absorber!(
    BCS, "Black Construction Paper", 0.83, 0.9;
    RB, "Red Brick", 0.75, 0.65
);

impl_reflective!(AF, "Aluminum Foil", 0.55, 0.7);

impl_insulator!(Newspaper, "Newspaper", 0., 0.123);

impl_body!(Cardboard, "Cardboard", 1.75, 0.064, 0.004);
