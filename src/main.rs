mod cost;
mod design;
mod materials;
mod reflectors;
mod trendline;
use argmin::core::{CostFunction, Error, Executor, Gradient, State};
use argmin::solver::gradientdescent::SteepestDescent;
use argmin::solver::linesearch::MoreThuenteLineSearch;
use std::fmt::Display;

pub use design::Design;

pub const SOLAR_POWER_DENSITY: f64 = 1000.;
pub const AMBIENT: f64 = 21.;
/// 50 degrees in radians
pub const SUN_ANGLE: f64 = 0.872664626;
/// pi/2 - sun_angle (in radians)
pub const OVEN_ANGLE: f64 = 1.5707963268 - SUN_ANGLE;

struct Oven {}

#[derive(Debug)]
enum OvenError {
    InvalidParameters,
}

impl std::error::Error for OvenError {}

impl Display for OvenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OvenError::InvalidParameters => write!(f, "Invalid parameters"),
        }
    }
}

impl CostFunction for Oven {
    type Param = Design;
    type Output = f64;

    fn cost(&self, design: &Self::Param) -> Result<Self::Output, argmin::core::Error> {
        if !design.ok() {
            return Err(Box::new(OvenError::InvalidParameters).into());
        }

        let tio = design.predicted_tio();

        Ok(tio.recip())
    }
}

fn main() {
    let design = Design {
        absorber: materials::Absorber::BlackConstructionPaper,
        window: materials::WindowMaterial::DoubleMylar,
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
        reflector_ml: 3.,
        reflectors: materials::ReflectiveMaterial::TinFoil,
    };

    println!(
        "tio: {} with cost ${}",
        design.predicted_tio(),
        design.total_cost()
    );
}
