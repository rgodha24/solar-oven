use argmin_math::*;
use ndarray::Array4;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, Index, IndexMut};

/// [l_and_w, h, insulator_thickness, reflector_ml]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input(pub [f64; 4]);

impl Index<usize> for Input {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Input {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Deref for Input {
    type Target = [f64; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ArgminSub<Input, Input> for Input {
    fn sub(&self, other: &Input) -> Input {
        let mut result = [0.0; 4];
        for i in 0..4 {
            result[i] = self[i] - other[i];
        }
        Input(result)
    }
}

impl ArgminMul<f64, Input> for Input {
    fn mul(&self, other: &f64) -> Input {
        let mut result = [0.0; 4];
        for i in 0..4 {
            result[i] = self[i] * other;
        }
        Input(result)
    }
}

impl ArgminDot<Input, f64> for Input {
    fn dot(&self, other: &Input) -> f64 {
        let mut result = 0.0;
        for i in 0..4 {
            result += self[i] * other[i];
        }
        result
    }
}

impl ArgminAdd<Input, Input> for Input {
    fn add(&self, other: &Input) -> Input {
        let mut result = [0.0; 4];
        for i in 0..4 {
            result[i] = self[i] + other[i];
        }
        Input(result)
    }
}

impl ArgminMul<Input, Input> for f64 {
    fn mul(&self, other: &Input) -> Input {
        let mut result = [0.0; 4];
        for i in 0..4 {
            result[i] = self * other[i];
        }
        Input(result)
    }
}

impl ArgminDot<Input, Input> for f64 {
    fn dot(&self, other: &Input) -> Input {
        let mut result = [0.0; 4];
        for i in 0..4 {
            result[i] = self * other[i];
        }
        Input(result)
    }
}

impl ArgminL2Norm<f64> for Input {
    fn l2_norm(&self) -> f64 {
        let mut result = 0.0;
        for i in 0..4 {
            result += self[i] * self[i];
        }
        result.sqrt()
    }
}
