use std::fmt::Display;

use crate::*;
use linreg::linear_regression;

#[derive(Debug, Clone)]
pub struct Design {
    pub absorber: Absorber,
    pub l_and_w: f64,
    pub h: f64,
    pub inner_body: BodyMaterial,
    pub insulator: Insulator,
    pub insulator_thickness: f64,
    pub outer_body: BodyMaterial,
    pub window: WindowMaterial,
    pub reflectors: ReflectiveMaterial,
    pub reflector_count: u8,
    pub reflector_ml: f64,
    pub reflector_type: ReflectorType,
}

impl Design {
    pub fn ok(&self) -> bool {
        self.l_and_w > 0.
            && self.h > 0.
            && self.insulator_thickness > 0.
            && self.reflector_count > 0
            && self.reflector_ml > 0.
            && (self.reflector_ml - 3.).abs() < 0.001
            && (self.chamber_volume() - 0.001).abs() < 1e-8
    }

    fn chamber_volume(&self) -> f64 {
        self.l_and_w * self.l_and_w * self.h
    }
    fn usb(&self) -> f64 {
        // (x1/k1 + x2/k2 + x3/k3)^-1

        let iw = self.inner_body.thickness() / self.inner_body.conductivity();
        let c = self.insulator_thickness / self.insulator.conductivity();
        let ow = self.outer_body.thickness() / self.outer_body.conductivity();

        let sum = iw + c + ow;

        sum.recip()
    }

    fn transmitivity(&self) -> f64 {
        self.window.transmitivity()
    }

    fn absorptivity(&self) -> f64 {
        self.absorber.absoptivity()
    }

    fn tio_at_uw(&self, uw: f64) -> f64 {
        let num = SOLAR_POWER_DENSITY
            * self.aw()
            * self.transmitivity()
            * self.absorptivity()
            * (OVEN_ANGLE + SUN_ANGLE).sin()
            * self.gain();

        let denom = self.usb() * self.asb() + uw * self.aw();

        AMBIENT + num / denom
    }

    pub(crate) fn aw(&self) -> f64 {
        self.l_and_w * self.l_and_w
    }

    pub(crate) fn asb(&self) -> f64 {
        self.aw() + 4. * self.h * self.l_and_w
    }

    pub fn tio_line(&self) -> Result<LNTrendline, ()> {
        // x
        let uws: Vec<_> = self.window.uws().into_iter().map(|n| n.0.ln()).collect();

        // y
        let tios: Vec<_> = self
            .window
            .uws()
            .into_iter()
            .map(|n| n.0)
            .map(|n| self.tio_at_uw(n))
            .collect();

        let (a, b) = linear_regression(&uws, &tios).map_err(|_| ())?;

        Ok(LNTrendline {
            coefficient: a,
            intercept: b,
        })
    }

    pub fn predicted_tio(&self) -> f64 {
        let tio_line = self.tio_line();
        let window_line = self.window.uw_line();

        tio_line
            .map(|l| l.y_intercept(&window_line))
            .unwrap_or(AMBIENT)
    }

    /// lower is better
    pub fn score(&self) -> f64 {
        let tio = self.predicted_tio();
        let cost = self.total_cost();

        // we want to get to at least 400F to cook the food correctly
        (GOAL_TIO - tio).abs() + cost * 4.

        // ((tio - AMBIENT) / cost).recip()

        // tio.recip()
    }

    pub fn performance_index(&self) -> f64 {
        let tio = self.predicted_tio();
        let cost = self.total_cost();

        (tio - AMBIENT) / cost
    }
}

impl Display for Design {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Absorber: {}", self.absorber)?;
        writeln!(f, "L and W: {:.4} cm", self.l_and_w * 100.)?;
        writeln!(f, "H: {:.4} cm", self.h * 100.)?;
        writeln!(f, "Inner Body: {:.4}", self.inner_body)?;
        writeln!(f, "Insulator: {}", self.insulator)?;
        writeln!(
            f,
            "Insulator Thickness: {:.4} cm",
            self.insulator_thickness * 100.
        )?;
        writeln!(f, "Outer Body: {:.4}", self.outer_body)?;
        writeln!(f, "Window: {}", self.window)?;
        writeln!(f, "Reflectors: {}", self.reflectors)?;
        writeln!(f, "Reflector Count: {}", self.reflector_count)?;
        writeln!(f, "Reflector ML: {:.4}", self.reflector_ml)?;
        writeln!(f, "Reflector Type: {:?}", self.reflector_type)?;
        writeln!(f, "Cost: ${:.4}", self.total_cost())?;
        writeln!(f, "Temp: {:.4}°C", self.predicted_tio())
    }
}

impl From<(&Oven, &[f64])> for Design {
    fn from((oven, x): (&Oven, &[f64])) -> Self {
        Design {
            absorber: oven.abs,
            window: oven.window,
            l_and_w: (0.001 / x[0]).sqrt(),
            h: x[0],
            outer_body: oven.outer_body,
            inner_body: oven.inner_body,
            insulator: oven.insulator,
            insulator_thickness: x[1],
            reflector_count: oven.reflector_number,
            reflector_type: oven.reflector_type,
            reflector_ml: x[2],
            reflectors: oven.reflective_material,
        }
    }
}
