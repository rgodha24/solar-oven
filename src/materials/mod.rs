mod window;

use std::fmt::Display;

pub use window::*;

#[derive(Debug, Clone, Copy)]
pub enum ReflectiveMaterial {
    TinFoil,
}

#[derive(Debug, Clone, Copy)]
pub enum Insulator {
    Newspaper,
}

#[derive(Debug, Clone, Copy)]
pub enum BodyMaterial {
    Cardboard,
}

#[derive(Debug, Clone, Copy)]
pub enum Absorber {
    BlackConstructionPaper,
}

impl ReflectiveMaterial {
    pub fn cost_per_m2(&self, m2: f64) -> f64 {
        match self {
            Self::TinFoil => 0.55 * m2,
        }
    }

    pub fn reflectivity(&self) -> f64 {
        match self {
            Self::TinFoil => 0.7,
        }
    }

    pub fn variants() -> &'static [Self] {
        &[Self::TinFoil]
    }
}

impl Absorber {
    pub fn absoptivity(&self) -> f64 {
        match self {
            Self::BlackConstructionPaper => 0.9,
        }
    }

    pub fn cost_per_m2(&self, m2: f64) -> f64 {
        match self {
            Self::BlackConstructionPaper => 0.5 * m2,
        }
    }
    pub fn variants() -> &'static [Self] {
        &[Self::BlackConstructionPaper]
    }
}

impl Insulator {
    pub fn conductivity(&self) -> f64 {
        match self {
            Self::Newspaper => 0.123,
        }
    }
    pub fn cost_per_m3(&self, volume: f64) -> f64 {
        match self {
            Self::Newspaper => 0.,
        }
    }
    pub fn variants() -> &'static [Self] {
        &[Self::Newspaper]
    }
}

impl BodyMaterial {
    pub fn conductivity(&self) -> f64 {
        match self {
            BodyMaterial::Cardboard => 0.064,
        }
    }
    pub fn cost_per_m2(&self, m2: f64) -> f64 {
        match self {
            Self::Cardboard => 1.75 * m2,
        }
    }
    pub fn variants() -> &'static [Self] {
        &[Self::Cardboard]
    }
    pub fn thickness(&self) -> f64 {
        match self {
            Self::Cardboard => 0.004,
        }
    }
}

impl Display for ReflectiveMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReflectiveMaterial::TinFoil => write!(f, "Tin Foil"),
        }
    }
}

impl Display for BodyMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyMaterial::Cardboard => write!(f, "Cardboard"),
        }
    }
}

impl Display for Insulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Insulator::Newspaper => write!(f, "Newspaper"),
        }
    }
}
