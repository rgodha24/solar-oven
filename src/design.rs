use crate::trendline::LNTrendline;
use crate::{materials::*, AMBIENT, OVEN_ANGLE, SOLAR_POWER_DENSITY, SUN_ANGLE};
use linreg::linear_regression;

#[derive(Debug, Clone)]
pub struct Design<
    RT: ReflectorType,
    IBM: BodyMaterial,
    OBM: BodyMaterial,
    IM: Insulator,
    WM: Window,
    RM: ReflectiveMaterial,
    AM: Absorber,
> {
    pub absorber: AM,
    pub l_and_w: f64,
    pub h: f64,
    pub inner_body: IBM,
    pub inner_body_thickness: f64,
    pub insulator: IM,
    pub insulator_thickness: f64,
    pub outer_body: OBM,
    pub outer_body_thickness: f64,
    pub window: WM,
    pub reflectors: RM,
    pub reflector_count: u8,
    pub reflector_ml: f64,
    pub reflector_type: RT,
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

        let iw = self.inner_body_thickness / IBM::CONDUCTIVITY;
        let c = self.insulator_thickness / IM::CONDUCTIVITY;
        let ow = self.outer_body_thickness / OBM::CONDUCTIVITY;

        let sum = iw + c + ow;

        sum.recip()
    }

    fn transmitivity(&self) -> f64 {
        WM::TRANSMITIVITY
    }

    fn absorptivity(&self) -> f64 {
        AM::ABSORPTIVITY
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
        let uws: Vec<_> = WM::UWS.iter().map(|n| n.0.ln()).collect();

        // y
        let tios: Vec<_> = WM::UWS
            .iter()
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
        let window_line = WM::TRENDLINE;

        tio_line.y_intercept(&window_line)
    }
}
