mod macros;
mod window;

use crate::{impl_absorber, impl_body, impl_insulator, impl_reflective};
pub use window::*;

// for all of these:
// enum name, pretty name, cost, ...

impl_absorber!(
    BCS, "Black Construction Paper", 0.83, 0.9;
    // RB, "Red Brick", 0.75, 0.65;
    TSC, "Thurmalox Solar Coating", 38.95, 0.96
);

impl_reflective!(
    AF, "Aluminum Foil", 0.55, 0.7;
    TF, "Tin Foil", 0.68, 0.88;
    MS, "Mirror Sheets", 23.24, 0.9;
    RT, "Reflective Tape", 19.35, 0.85;
    // S, "Silver", 79.36, 0.964;
    // RV, "Reflective Vinyl", 16.55, 0.8;
    SR2000, "S Reflect 2000", 29.95, 0.92;
    SRV, "Silver Reflective Vinyl", 13.40, 0.86
);

impl_insulator!(
    N, "Newspaper", 0., 0.123;
    FG, "Fiberglass Insulation", 83.15, 0.076923077;
    SF, "Spray Foam", 19612.13, 0.039;
    DF, "Down Feather", 485.57, 0.045;
    FG30, "R30 Fiberglass", 76.60, 0.03333333333
);

impl_body!(
    C, "Cardboard", 1.75, 0.064, 0.004;
    // W5, "Wood .5in", 2.48, 0.15, 0.5;
    // W2, "Wood .2in", 2.48, 0.15, 0.2;
    W16, "Wood 1/16 in", 2.48, 0.15, (16f64).recip()
);
