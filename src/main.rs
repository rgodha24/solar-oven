mod cost;
mod design;
mod materials;
mod trendline;

use crate::materials::*;
use argmin::core::CostFunction;
use itertools::iproduct;
use std::fmt::Display;

pub use design::Design;

pub const SOLAR_POWER_DENSITY: f64 = 1000.;
pub const AMBIENT: f64 = 21.;
/// 50 degrees in radians
pub const SUN_ANGLE: f64 = 0.872664626;
/// pi/2 - sun_angle (in radians)
pub const OVEN_ANGLE: f64 = 1.5707963268 - SUN_ANGLE;

struct Oven<RT, IBM, OBM, IM, WM, RM, AM> {
    _phantom: std::marker::PhantomData<(RT, IBM, OBM, IM, WM, RM, AM)>,
}

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

impl<RT, IBM, OBM, IM, WM, RM, AM> CostFunction for Oven<RT, IBM, OBM, IM, WM, RM, AM>
where
    RT: ReflectorType,
    IBM: BodyMaterial,
    OBM: BodyMaterial,
    IM: Insulator,
    WM: Window,
    RM: ReflectiveMaterial,
    AM: Absorber,
{
    type Param = Design<RT, IBM, OBM, IM, WM, RM, AM>;
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
        absorber: BlackConstructionPaper {},
        inner_body: Cardboard {},
        reflectors: TinFoil {},
        h: 0.1,
        l_and_w: 0.85,
        window: SingleMylar {},
        insulator: Newspaper {},
        outer_body: Cardboard {},
        reflector_ml: 2.,
        reflector_type: Rectangular {},
        reflector_count: 4,
        insulator_thickness: 0.135,
        inner_body_thickness: 0.04,
        outer_body_thickness: 0.04,
    };

    let absorbers = vec![BlackConstructionPaper {}].into_iter();
    let inner_bodies = vec![Cardboard {}].into_iter();
    let reflectors = vec![TinFoil {}].into_iter();
    let windows = vec![SingleMylar {}].into_iter();
    let insulators = vec![Newspaper {}].into_iter();
    let outer_bodies = vec![Cardboard {}].into_iter();
    let reflector_types = vec![Rectangular {}].into_iter();

    let iter = iproduct!(
        absorbers,
        inner_bodies,
        reflectors,
        windows,
        insulators,
        outer_bodies,
        reflector_types
    );

    for (a, ib, r, w, ins, ob, rt) in iter {
        let design = Design {
            absorber: a,
            inner_body: ib,
            reflectors: r,
            h: 0.1,
            l_and_w: 0.85,
            window: w,
            insulator: ins,
            outer_body: ob,
            reflector_ml: 2.,
            reflector_type: rt,
            reflector_count: 4,
            insulator_thickness: 0.135,
            inner_body_thickness: 0.04,
            outer_body_thickness: 0.04,
        };

        if design.ok() {
            println!(
                "predicted tio: {} cost: ${}",
                design.predicted_tio(),
                design.total_cost()
            );
        }
    }

    println!(
        "predicted tio: {} cost: ${}",
        design.predicted_tio(),
        design.total_cost()
    );
}
