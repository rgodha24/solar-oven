mod materials;
mod trendline;

use std::fmt::Display;

use materials::*;

struct Design {
    absorber: Absorber,
    l_and_w: f64,
    h: f64,
    inner_body: BodyMaterial,
    inner_body_thickness: f64,
    insulator: Insulator,
    insulator_thickness: f64,
    outer_body: BodyMaterial,
    outer_body_thickness: f64,
    window: WindowMaterial,
    reflectors: ReflectiveMaterial,
    reflector_count: u8,
    reflector_ml: f64,
    reflector_l: f64,
    reflector_type: ReflectorType,
}

enum ReflectorType {
    Rectangular,
    // Trapezoidal
}

impl ReflectorType {
    /// ALPHA IS IN RADIANS !!
    pub fn calc_gain(&self, count: u8, reflectivity: f64, reflector_ml: f64, alpha: f64) -> f64 {
        match self {
            Self::Rectangular => 1. + (count as f64) * reflectivity * reflector_ml * alpha.sin(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
