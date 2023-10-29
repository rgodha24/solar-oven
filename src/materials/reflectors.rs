use crate::materials::*;
use crate::Design;

pub trait ReflectorType {
    fn area(&self, ml: f64, window: f64, n: u8) -> f64;
    fn calc_gain(&self, count: u8, reflectivity: f64, reflector_ml: f64, alpha: f64) -> f64;
}

pub trait ReflectiveMaterial {
    const REFLECTIVITY: f64;
    fn cost_per_m2(&self, m2: f64) -> f64;
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangular {}

#[derive(Debug, Clone, Copy)]
pub struct TinFoil {}

impl ReflectorType for Rectangular {
    fn area(&self, ml: f64, window: f64, n: u8) -> f64 {
        (ml * window) * window * (n as f64)
    }
    fn calc_gain(&self, count: u8, reflectivity: f64, reflector_ml: f64, alpha: f64) -> f64 {
        1. + (count as f64) * reflectivity * reflector_ml * alpha.sin()
    }
}

impl ReflectiveMaterial for TinFoil {
    const REFLECTIVITY: f64 = 0.7;

    fn cost_per_m2(&self, m2: f64) -> f64 {
        m2 * 0.55
    }
}

impl<RT, IBM, OBM, IM, WM, RM, AM> Design<RT, IBM, OBM, IM, WM, RM, AM>
where
    RT: ReflectorType,
    IBM: BodyMaterial,
    OBM: BodyMaterial,
    IM: Insulator,
    WM: Window,
    RM: ReflectiveMaterial,
    AM: Absorber,
{
    pub fn gain(&self) -> f64 {
        self.reflector_type.calc_gain(
            self.reflector_count,
            RM::REFLECTIVITY,
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
