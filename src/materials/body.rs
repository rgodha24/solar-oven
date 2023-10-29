pub trait BodyMaterial {
    const CONDUCTIVITY: f64;

    fn cost_per_m2(&self, m2: f64) -> f64;
}

#[derive(Debug, Clone, Copy)]
pub struct Cardboard {}

impl BodyMaterial for Cardboard {
    const CONDUCTIVITY: f64 = 0.064;

    fn cost_per_m2(&self, m2: f64) -> f64 {
        1.75 * m2
    }
}
