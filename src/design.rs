use crate::reflectors::ReflectorType;
use crate::trendline::LNTrendline;
use crate::{materials::*, AMBIENT, OVEN_ANGLE, SOLAR_POWER_DENSITY, SUN_ANGLE};
use linreg::linear_regression;

#[derive(Debug, Clone)]
pub struct Design {
    pub absorber: Absorber,
    pub l_and_w: f64,
    pub h: f64,
    pub inner_body: BodyMaterial,
    pub inner_body_thickness: f64,
    pub insulator: Insulator,
    pub insulator_thickness: f64,
    pub outer_body: BodyMaterial,
    pub outer_body_thickness: f64,
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
            && self.inner_body_thickness > 0.
            && self.insulator_thickness > 0.
            && self.outer_body_thickness > 0.
            && self.reflector_count > 0
            && self.reflector_ml > 0.
            && self.chamber_volume() > 0.001
    }

    fn chamber_volume(&self) -> f64 {
        self.l_and_w * self.l_and_w * self.h
    }
    fn usb(&self) -> f64 {
        // (x1/k1 + x2/k2 + x3/k3)^-1

        let iw = self.inner_body_thickness / self.inner_body.conductivity();
        let c = self.insulator_thickness / self.insulator.conductivity();
        let ow = self.outer_body_thickness / self.outer_body.conductivity();

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

    // TODO: does this work with only 2 tested numbers
    pub fn tio_line(&self) -> LNTrendline {
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

        let (a, b) = linear_regression(&uws, &tios).unwrap();

        LNTrendline {
            coefficient: a,
            intercept: b,
        }
    }

    pub fn predicted_tio(&self) -> f64 {
        let tio_line = self.tio_line();
        let window_line = self.window.uw_line();

        tio_line.y_intercept(&window_line)
    }
}

#[cfg(test)]
mod tests {

    fn with_window(window: materials::WindowMaterial) -> Design {
        Design {
            absorber: materials::Absorber::BlackConstructionPaper,
            window,
            l_and_w: 0.085,
            h: 0.1,
            outer_body: materials::BodyMaterial::Cardboard,
            outer_body_thickness: 0.004,
            inner_body: materials::BodyMaterial::Cardboard,
            inner_body_thickness: 0.004,
            insulator: materials::Insulator::Newspaper,
            insulator_thickness: 0.135,
            reflector_count: 4,
            reflector_type: reflectors::ReflectorType::Rectangular,
            reflector_ml: 2.,
            reflectors: materials::ReflectiveMaterial::TinFoil,
        }
    }

    use crate::{assert_float_eq, materials, reflectors};

    use super::*;
    #[test]
    fn with_provided_uw() {
        let design = with_window(WindowMaterial::SingleMylar);

        let expected = 192.00020164848100;
        let calc = design.tio_at_uw(10.10);
        assert_float_eq!(expected, calc);

        let design = with_window(WindowMaterial::DoubleMylar);

        let expected = 264.33652371572700;
        let calc = design.tio_at_uw(4.88);
        assert_float_eq!(expected, calc);
    }

    #[test]
    fn trendline() {
        let design = with_window(WindowMaterial::SingleMylar);
        let trend = design.tio_line();

        let expected_coefficient = -82.94265852436450;
        let expected_intercept = 377.42095667476300;

        println!("{:?}", trend);

        assert_float_eq!(trend.coefficient, expected_coefficient);
        assert_float_eq!(trend.intercept, expected_intercept);
    }

    #[test]
    fn all() {
        let design = with_window(WindowMaterial::SingleMylar);

        let expected = 129.9165902561410;
        let calculated = design.predicted_tio();

        assert_float_eq!(expected, calculated);
    }
}

#[macro_export]
macro_rules! assert_float_eq {
    ($left:expr, $right:expr) => {
        assert_float_eq!($left, $right, 1e-6);
    };

    ($left:expr, $right:expr, $precision:expr) => {
        println!("{} {}", $left, $right);
        let abs = ($right - $left).abs();

        assert!(abs < $precision);
    };
}
