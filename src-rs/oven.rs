use cobyla::{minimize, Func, RhoBeg};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{
    Absorber, BodyMaterial, Design, Insulator, ReflectiveMaterial, ReflectorType, WindowMaterial,
};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Oven(
    pub Absorber,
    pub WindowMaterial,
    pub BodyMaterial,
    pub BodyMaterial,
    pub Insulator,
    pub ReflectorType,
    pub ReflectiveMaterial,
    pub u8,
);

pub fn score(input: &[f64], oven: &mut Oven) -> f64 {
    let design: Design = (&*oven, input).into();

    design.score()
}

impl Oven {
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

    /// returns (h, insulator_thickness, data specifified by GraphDataResponse)
    pub fn graph_data(
        &self,
        reflector_ml: f64,
        response_type: GraphDataResponse,
    ) -> Vec<(f64, f64, f64)> {
        let mut data = Vec::new();
        // 0.05 <= h <= 0.4
        for h in (5..=40).map(|n| (n as f64) / 100.) {
            for insulator_thickness in (0..=150).map(|n| (n as f64) / 100.) {
                let input: &[f64] = &[h, insulator_thickness, reflector_ml];
                let design: Design = (&*self, input).into();

                let z = match response_type {
                    GraphDataResponse::Tio => design.predicted_tio(),
                    GraphDataResponse::Cost => design.total_cost(),
                    GraphDataResponse::PerformanceIndex => design.performance_index(),
                    GraphDataResponse::Score => design.score(),
                };

                data.push((h, insulator_thickness, z));
            }
        }

        data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum GraphDataResponse {
    Tio,
    Cost,
    PerformanceIndex,
    Score,
}
