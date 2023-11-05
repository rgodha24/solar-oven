use crate::Design;

#[derive(Debug, Clone, Copy)]
pub enum ReflectorType {
    Rectangular,
    // Trapezoidal
}

impl ReflectorType {
    /// ALPHA IS IN RADIANS !!
    pub fn calc_gain(&self, count: u8, reflectivity: f64, reflector_ml: f64, alpha: f64) -> f64 {
        match self {
            Self::Rectangular => 1. + (count as f64) * reflectivity * reflector_ml * alpha.sin(),
        }
    }

    pub fn area(&self, ml: f64, window: f64, n: u8) -> f64 {
        match self {
            Self::Rectangular => (ml * window) * window * (n as f64),
        }
    }

    pub fn variants() -> &'static [Self] {
        &[Self::Rectangular]
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
