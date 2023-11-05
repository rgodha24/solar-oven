mod cost;
mod design;
mod materials;
mod reflectors;
mod trendline;
use argmin::core::{CostFunction, Error, Executor, Gradient, State};
use cobyla::{minimize, Func, RhoBeg};
use indicatif::ProgressBar;
use std::{fmt::Display, io::Write};

use materials::*;

pub use design::Design;

use crate::reflectors::ReflectorType;

/// [l_and_w, h, insulator_thickness, reflector_ml]
type Input = [f64; 4];

pub const SOLAR_POWER_DENSITY: f64 = 1000.;
pub const AMBIENT: f64 = 21.;
/// 50 degrees in radians
pub const SUN_ANGLE: f64 = 0.872664626;
/// pi/2 - sun_angle (in radians)
pub const OVEN_ANGLE: f64 = 1.5707963268 - SUN_ANGLE;

#[derive(Debug, Clone)]
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

// diff function checks if the oven is "ok"
fn score(input: &[f64], (oven, bar): &mut (Oven, ProgressBar)) -> f64 {
    let design = Design {
        absorber: oven.0.clone(),
        window: oven.1.clone(),
        l_and_w: input[0],
        h: input[1],
        outer_body: oven.3.clone(),
        inner_body: oven.2.clone(),
        insulator: oven.4.clone(),
        insulator_thickness: input[2],
        reflector_count: oven.7,
        reflector_type: oven.5.clone(),
        reflector_ml: input[3],
        reflectors: oven.6.clone(),
    };
    bar.inc(1);

    design.score().recip()
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
        println!("Oven: {:?}", oven);
        let mut init: Input = [0.1, 0.1, 0.1, 3.];

        let volume = |x: &[f64], (_oven, _bar): &mut (Oven, ProgressBar)| {
            if (x[0] * x[1] * x[1]) < 0.1 {
                1.
            } else {
                -1.
            }
        };
        let nan = |x: &[f64], (_oven, _bar): &mut (Oven, ProgressBar)| {
            if x[0].is_nan() || x[1].is_nan() || x[2].is_nan() || x[3].is_nan() {
                -1.
            } else {
                1.
            }
        };
        let cons: Vec<&dyn Func<(Oven, ProgressBar)>> = vec![&volume, &nan];

        let bar = ProgressBar::new(10_000_000);

        let (x, y) = match minimize(
            score,
            &mut init,
            &[(1e-2, 0.25), (1e-2, 0.25), (1e-2, 1.), (1e-2, 3.)],
            &cons,
            (oven.clone(), bar.clone()),
            10_000_000,
            RhoBeg::All(0.5),
            None,
        ) {
            Ok((_, x, y)) => Ok((x, y)),
            Err((cobyla::FailStatus::RoundoffLimited, x, y)) => Ok((x, y)),
            Err((e, _, _)) => Err(e),
        }
        .unwrap();

        bar.finish();
        let best_design = Design {
            absorber: oven.0,
            window: oven.1,
            l_and_w: x[0],
            h: x[1],
            outer_body: oven.3,
            inner_body: oven.2,
            insulator: oven.4,
            insulator_thickness: x[2],
            reflector_count: oven.7,
            reflector_type: oven.5,
            reflector_ml: x[3],
            reflectors: oven.6,
        };

        println!("Best design: {:?}", best_design);
        println!("Score: {}", y);
        println!("Cost: {}", best_design.total_cost());
        println!("Temperature: {}", best_design.predicted_tio());
    }
}
