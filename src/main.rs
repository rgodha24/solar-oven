mod cost;
mod design;
mod materials;
mod reflectors;
mod trendline;
use std::sync::{Arc, Mutex};

use cobyla::{minimize, Func, RhoBeg};
use indicatif::ProgressBar;
use rayon::prelude::*;

use materials::*;

pub use design::Design;

use crate::reflectors::ReflectorType;

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
    let all = Arc::new(Mutex::new(Vec::new()));

    let initials: Vec<[f64; 3]> = vec![[0.1, 0.1, 3.], [0.4, 1.5, 3.]];

    let variants: Vec<_> = (itertools::iproduct![
        Absorber::variants(),
        WindowMaterial::variants(),
        BodyMaterial::variants(),
        BodyMaterial::variants(),
        Insulator::variants(),
        ReflectorType::variants(),
        ReflectiveMaterial::variants(),
        1..=4u8,
        initials
    ])
    .collect();

    let variant_count = variants.len();
    let pb = ProgressBar::new(variant_count as u64);

    variants.into_par_iter().for_each_with(
        all.clone(),
        |all, (&a, &w, &ob, &ib, &ins, &rt, &rm, rn, mut init)| {
            let oven = Oven(a, w, ob, ib, ins, rt, rm, rn);

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
                &[(5e-2, 0.4), (0., 1.5), (1e-2, 3.)],
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

            all.lock().unwrap().push((best_design, y));

            pb.inc(1);
        },
    );

    pb.finish();

    let mut all = all.lock().unwrap();
    all.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!("\nBest 3 designs of {variant_count}:\n");
    for (design, score) in all.iter().take(3) {
        println!("score: {score}");
        println!(
            "cost based performance index: {}",
            (design.predicted_tio() - AMBIENT) / design.total_cost()
        );
        println!("{}", design);
    }
}
