#[cfg(not(target_arch = "wasm32"))]
use cobyla::{minimize, Func, RhoBeg};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{
    Absorber, BodyMaterial, Design, Insulator, ReflectiveMaterial, ReflectorType, WindowMaterial,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
#[cfg_attr(not(target_arch = "wasm32"), derive(specta::Type))]
pub struct Oven {
    pub abs: Absorber,
    pub window: WindowMaterial,
    pub inner_body: BodyMaterial,
    pub outer_body: BodyMaterial,
    pub insulator: Insulator,
    pub reflector_type: ReflectorType,
    pub reflective_material: ReflectiveMaterial,
    pub reflector_number: u8,
}

pub fn score(input: &[f64], oven: &mut Oven) -> f64 {
    let design: Design = (&*oven, input).into();

    design.score()
}

impl Oven {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn best_design(self, mut init: [f64; 3]) -> (Design, f64) {
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
            self.clone(),
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

        ((&self, &x[..]).into(), y)
    }
}

#[wasm_bindgen]
pub fn oven_from_json(oven: &str) -> Option<Oven> {
    serde_json::from_str(oven).ok()
}

/// returns (h, insulator_thickness, data specifified by GraphDataResponse)
#[wasm_bindgen]
pub fn graph_data(
    oven: &Oven,
    reflector_ml: f64,
    response_type: GraphDataResponse,
) -> Vec<GraphData> {
    let mut data = Vec::new();
    // 0.05 <= h <= 0.4
    for h in (5..=40).map(|n| (n as f64) / 100.) {
        for insulator_thickness in (0..=150).map(|n| (n as f64) / 100.) {
            let input: &[f64] = &[h, insulator_thickness, reflector_ml];
            let design: Design = (&*oven, input).into();

            let z = match response_type {
                GraphDataResponse::Tio => design.predicted_tio(),
                GraphDataResponse::Cost => design.total_cost(),
                GraphDataResponse::PerformanceIndex => design.performance_index(),
                GraphDataResponse::Score => design.score(),
            };

            data.push(GraphData {
                h,
                insulator_thickness,
                z,
            });
        }
    }

    data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct GraphData {
    pub h: f64,
    pub insulator_thickness: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum GraphDataResponse {
    Tio,
    Cost,
    PerformanceIndex,
    Score,
}
