use crate::trendline::LNTrendline;

#[derive(Debug, Clone)]
pub enum WindowMaterial {
    SingleMylar,
    DoubleMylar,
}

type UWGraph = (f64, f64);

impl WindowMaterial {
    pub fn cost_per_m2(&self, m2: f64) -> f64 {
        match self {
            // TODO: find out the size of the mylar sheets
            WindowMaterial::SingleMylar => 0.25,
            WindowMaterial::DoubleMylar => 0.5,
        }
    }

    /// Vec<(Uw, Temperature)>
    pub fn uws(&self) -> Vec<UWGraph> {
        match self {
            WindowMaterial::SingleMylar => vec![
                (10.10, 66.),
                (13.90, 93.),
                (18.66, 121.),
                (24.34, 149.),
                (31.60, 177.),
                (40.11, 204.),
            ],
            WindowMaterial::DoubleMylar => vec![
                (4.88, 66.),
                (6.69, 93.),
                (8.96, 121.),
                (11.74, 149.),
                (15.20, 177.),
                (19.35, 204.),
            ],
        }
    }

    /// y = temperature, x = uw
    pub fn uw_line(&self) -> LNTrendline {
        match self {
            WindowMaterial::SingleMylar => LNTrendline {
                coefficient: 100.5092939,
                intercept: -170.0073519,
            },
            WindowMaterial::DoubleMylar => LNTrendline {
                coefficient: 100.5953534,
                intercept: -96.79716142,
            },
        }
    }

    pub fn transmitivity(&self) -> f64 {
        match self {
            WindowMaterial::SingleMylar => 0.92,
            WindowMaterial::DoubleMylar => (0.92_f64).powi(2),
        }
    }
}
