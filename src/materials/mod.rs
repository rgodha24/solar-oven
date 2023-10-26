mod window;

pub use window::*;

pub enum ReflectiveMaterial {
    TinFoil,
}

pub enum Insulator {
    Newspaper,
}

pub enum BodyMaterial {
    Cardboard,
}

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
}

impl Insulator {
    pub fn conductivity(&self) -> f64 {
        match self {
            Self::Newspaper => 0.123,
        }
    }
    pub fn cost_per_volume(&self, volume: f64) -> f64 {
        match self {
            Self::Newspaper => 0.,
        }
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
}
