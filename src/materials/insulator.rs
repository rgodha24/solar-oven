pub trait Insulator {
    const CONDUCTIVITY: f64;
    fn cost_per_m3(&self, volume: f64) -> f64;
}

#[derive(Debug, Clone, Copy)]
pub struct Newspaper {}

impl Insulator for Newspaper {
    const CONDUCTIVITY: f64 = 0.123;

    fn cost_per_m3(&self, _volume: f64) -> f64 {
        0.
    }
}
