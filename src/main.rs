mod design;
mod materials;
mod reflectors;
mod trendline;

use std::fmt::Display;

pub use design::Design;

pub const SOLAR_POWER_DENSITY: f64 = 1000.;
pub const AMBIENT: f64 = 21.;
/// 50 degrees in radians
pub const SUN_ANGLE: f64 = 0.872664626;
/// pi/2 - sun_angle (in radians)
pub const OVEN_ANGLE: f64 = 1.5707963268 - SUN_ANGLE;

fn main() {
    let design = Design {
        absorber: materials::Absorber::BlackConstructionPaper,
        window: materials::WindowMaterial::SingleMylar,
        l_and_w: 0.085,
        h: 0.1,
        outer_body: materials::BodyMaterial::Cardboard,
        outer_body_thickness: 0.004,
        inner_body: materials::BodyMaterial::Cardboard,
        inner_body_thickness: 0.004,
        insulator: materials::Insulator::Newspaper,
        insulator_thickness: 0.135,
        reflector_count: 4,
        reflector_type: reflectors::ReflectorType::Rectangular,
        reflector_ml: 2.,
        reflectors: materials::ReflectiveMaterial::TinFoil,
        reflector_l: 0.85,
    };

    println!("predicted_tio: {}", design.predicted_tio());
}
