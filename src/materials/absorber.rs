pub trait Absorber {
    const ABSORPTIVITY: f64;
    fn cost_per_m2(&self, m2: f64) -> f64;
}

#[derive(Debug, Clone, Copy)]
pub struct BlackConstructionPaper {}

impl Absorber for BlackConstructionPaper {
    const ABSORPTIVITY: f64 = 0.9;

    fn cost_per_m2(&self, m2: f64) -> f64 {
        0.5 * m2
    }
}
