mod cost;
mod design;
mod materials;
mod oven;
mod reflectors;
mod trendline;

pub use design::Design;
pub use materials::*;
pub use oven::{score, Oven};
pub use reflectors::ReflectorType;
pub use trendline::LNTrendline;

use itertools::Itertools;

pub const SOLAR_POWER_DENSITY: f64 = 1000.;
pub const AMBIENT: f64 = 21.;
/// 50 degrees in radians
pub const SUN_ANGLE: f64 = 0.872664626;
/// pi/2 - sun_angle (in radians)
pub const OVEN_ANGLE: f64 = 1.5707963268 - SUN_ANGLE;
/// 400 F in Celsius
pub const GOAL_TIO: f64 = 204.4444444444;

const INITIALS: &'static [[f64; 3]] = &[[0.1, 0.1, 3.] /* [0.4, 1.5, 3.] */];

pub fn variants() -> Vec<(Oven, [f64; 3])> {
    (itertools::iproduct![
        Absorber::variants(),
        WindowMaterial::variants(),
        BodyMaterial::variants(),
        BodyMaterial::variants(),
        Insulator::variants(),
        ReflectorType::variants(),
        ReflectiveMaterial::variants(),
        1..=4u8
    ])
    .map(|(&a, &w, &ob, &ib, &ins, &rt, &rm, rn)| Oven(a, w, ob, ib, ins, rt, rm, rn))
    .cartesian_product(INITIALS.iter().cloned())
    .collect()
}