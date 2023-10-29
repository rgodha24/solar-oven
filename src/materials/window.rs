use crate::trendline::LNTrendline;

pub trait Window {
    /// Vec<(Uw, Temperature)>
    const UWS: &'static [UWGraph];

    const TRENDLINE: LNTrendline;

    const TRANSMITIVITY: f64;

    fn cost_per_m2(&self, m2: f64) -> f64;
}

#[derive(Debug, Clone, Copy)]
pub struct SingleMylar {}
#[derive(Debug, Clone, Copy)]
pub struct DoubleMylar {}

impl Window for SingleMylar {
    const UWS: &'static [UWGraph] = &[
        (10.10, 66.),
        (13.90, 93.),
        (18.66, 121.),
        (24.34, 149.),
        (31.60, 177.),
        (40.11, 204.),
    ];

    const TRENDLINE: LNTrendline = LNTrendline {
        coefficient: 100.5092939,
        intercept: -170.0073519,
    };

    const TRANSMITIVITY: f64 = 0.92;

    fn cost_per_m2(&self, m2: f64) -> f64 {
        0.25 * m2
    }
}

impl Window for DoubleMylar {
    const UWS: &'static [UWGraph] = &[
        (4.88, 66.),
        (6.69, 93.),
        (8.96, 121.),
        (11.74, 149.),
        (15.20, 177.),
        (19.35, 204.),
    ];

    const TRENDLINE: LNTrendline = LNTrendline {
        coefficient: 100.5953534,
        intercept: -96.79716142,
    };

    const TRANSMITIVITY: f64 = 0.8464;

    fn cost_per_m2(&self, m2: f64) -> f64 {
        0.5 * m2
    }
}

type UWGraph = (f64, f64);
