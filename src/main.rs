mod cost;
mod design;
mod input;
mod materials;
mod reflectors;
mod trendline;
use argmin::core::observers::{ObserverMode, SlogLogger};
use argmin::core::{CostFunction, Error, Executor, Gradient, State};
use argmin::solver::goldensectionsearch::GoldenSectionSearch;
use argmin::solver::gradientdescent::SteepestDescent;
use argmin::solver::linesearch::MoreThuenteLineSearch;
use argmin::solver::quasinewton::{BFGS, DFP};
use input::Input;
use std::fmt::Display;

use materials::*;

pub use design::Design;

use crate::reflectors::ReflectorType;

pub const SOLAR_POWER_DENSITY: f64 = 1000.;
pub const AMBIENT: f64 = 21.;
/// 50 degrees in radians
pub const SUN_ANGLE: f64 = 0.872664626;
/// pi/2 - sun_angle (in radians)
pub const OVEN_ANGLE: f64 = 1.5707963268 - SUN_ANGLE;

struct Oven(
    Absorber,
    WindowMaterial,
    BodyMaterial,
    BodyMaterial,
    Insulator,
    ReflectorType,
    ReflectiveMaterial,
    u8,
);

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
    type Param = Input;
    type Output = f64;

    fn cost(&self, input: &Self::Param) -> Result<Self::Output, argmin::core::Error> {
        let design = Design {
            absorber: self.0.clone(),
            window: self.1.clone(),
            l_and_w: input[0],
            h: input[1],
            outer_body: self.3.clone(),
            inner_body: self.2.clone(),
            insulator: self.4.clone(),
            insulator_thickness: input[2],
            reflector_count: self.7,
            reflector_type: self.5.clone(),
            reflector_ml: input[3],
            reflectors: self.6.clone(),
        };

        if !design.ok() {
            return Ok(999_999_999.);
        }

        Ok(design.score().recip())
    }
}

impl Gradient for Oven {
    type Param = Input;
    type Gradient = Input;

    fn gradient(&self, param: &Self::Param) -> Result<Self::Gradient, Error> {
        let mut grad = [0f64; 4];
        let h = 1e-8;
        for i in 0..4 {
            let mut p = param.clone();
            p[i] += h;
            grad[i] = (self.cost(&p)? - self.cost(param)?) / h;
        }
        Ok(Input(grad))
    }
}

fn main() {
    for (&a, &w, &ob, &ib, &ins, &rt, &rm, rn) in itertools::iproduct![
        Absorber::variants(),
        WindowMaterial::variants(),
        BodyMaterial::variants(),
        BodyMaterial::variants(),
        Insulator::variants(),
        ReflectorType::variants(),
        ReflectiveMaterial::variants(),
        1..=4
    ] {
        let oven = Oven(a, w, ob, ib, ins, rt, rm, rn);
        let mlts: MoreThuenteLineSearch<Input, Input, f64> = MoreThuenteLineSearch::new();
        let solver = SteepestDescent::new(mlts);

        let init: Input = Input([0.1, 0.1, 0.1, 3.]);

        let res = Executor::new(oven, solver)
            .configure(|state| state.param(init).max_iters(1_000_000))
            .add_observer(SlogLogger::term(), ObserverMode::Always)
            .run()
            .unwrap();

        println!(
            "{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} cost: {}",
            a,
            w,
            ob,
            ib,
            ins,
            rt,
            rm,
            rn,
            res.state().get_best_param().unwrap(),
            res.state().get_best_cost()
        );
    }
}
