use crate::Design;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
#[cfg_attr(not(target_arch = "wasm32"), derive(specta::Type))]
pub enum ReflectorType {
    Rectangular,
    Trapezoidal,
}

impl ReflectorType {
    /// ALPHA IS IN RADIANS !!
    pub fn calc_gain(&self, count: u8, reflectivity: f64, reflector_ml: f64, alpha: f64) -> f64 {
        match self {
            Self::Rectangular => 1. + (count as f64) * reflectivity * reflector_ml * alpha.sin(),
            Self::Trapezoidal => {
                1. + (count as f64)
                    * reflectivity
                    * reflector_ml
                    * alpha.sin()
                    * (1. + reflector_ml * alpha.sin())
            }
        }
    }

    pub fn area(&self, ml: f64, window: f64, n_reflectors: u8) -> f64 {
        let reflector_length = ml * window;
        match self {
            // reflector_length * reflector_width * n_reflectors
            Self::Rectangular => reflector_length * window * (n_reflectors as f64),
            Self::Trapezoidal => {
                ((reflector_length).powi(2) - window.powi(2)) * (n_reflectors as f64) / 4.
            }
        }
    }

    pub fn variants() -> &'static [Self] {
        &[Self::Rectangular, Self::Trapezoidal]
    }
}

impl Design {
    pub fn gain(&self) -> f64 {
        self.reflector_type.calc_gain(
            self.reflector_count,
            self.reflectors.reflectivity(),
            self.reflector_ml,
            self.alpha(),
        )
    }

    fn alpha(&self) -> f64 {
        let root = (self.reflector_ml.powi(2) + 8.).sqrt();
        let num = -self.reflector_ml + root;
        let n = num / 4.;

        n.asin()
    }
}
