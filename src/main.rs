mod cost;
mod design;
mod materials;
mod reflectors;
mod trendline;
use cobyla::{minimize, Func, RhoBeg};
use indicatif::{ProgressBar, ProgressIterator};

use materials::*;

pub use design::Design;

use crate::reflectors::ReflectorType;

/// h, insulator_thickness, reflector_ml
type Input = [f64; 3];

pub const SOLAR_POWER_DENSITY: f64 = 1000.;
pub const AMBIENT: f64 = 21.;
/// 50 degrees in radians
pub const SUN_ANGLE: f64 = 0.872664626;
/// pi/2 - sun_angle (in radians)
pub const OVEN_ANGLE: f64 = 1.5707963268 - SUN_ANGLE;
/// 400 F in Celsius
pub const GOAL_TIO: f64 = 204.4444444444;

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
fn score(input: &[f64], oven: &mut Oven) -> f64 {
    let design: Design = (&*oven, input).into();

    design.score()
}

fn main() {
    let mut all = Vec::new();

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
        let mut init = [0.1, 0.1, 3.];

        let nan = |x: &[f64], _oven: &mut Oven| {
            if x[0].is_nan() || x[1].is_nan() || x[2].is_nan() {
                -1.
            } else {
                1.
            }
        };

        let cons: Vec<&dyn Func<Oven>> = vec![&nan];

        let (x, y) = match minimize(
            score,
            &mut init,
            &[(5e-2, 0.4), (1e-4, 1.), (1e-2, 3.)],
            &cons,
            oven.clone(),
            100_000,
            RhoBeg::All(0.5),
            None,
        ) {
            Ok((_, x, y)) => Ok((x, y)),
            // found a better solution in < 100_000 iterations
            Err((cobyla::FailStatus::RoundoffLimited, x, y)) => Ok((x, y)),
            Err((e, _, _)) => Err(e),
        }
        .unwrap();

        let best_design: Design = (&oven, &x[..]).into();

        all.push((best_design, y));
    }

    all.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!("Best designs:\n");
    for (design, _) in all.iter().take(3) {
        println!(
            "cost based performance index: {}",
            (design.predicted_tio() - AMBIENT) / design.total_cost()
        );
        println!("{}", design);
    }
}
