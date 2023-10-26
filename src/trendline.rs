#[derive(Debug, Clone)]
pub struct LNTrendline {
    pub coefficient: f64,
    pub intercept: f64,
}

impl LNTrendline {
    pub fn x_intercept(&self, other: &LNTrendline) -> f64 {
        // self.coefficient * ln(x) + self.intercept = other.coefficient * ln(x) + other.intercept
        // (sc - oc)(ln(x)) = oi - si
        // ln(x) = (oi - si)/(sc - oc)
        // x = e ^ (oi - si)/(sc - oc)

        let sc = self.coefficient;
        let oc = other.coefficient;
        let si = self.intercept;
        let oi = other.intercept;

        let num = oi - si;
        let denom = sc - oc;

        let pow = num / denom;

        // e ^ pow
        pow.exp()
    }

    pub fn y_intercept(&self, other: &LNTrendline) -> f64 {
        let x_int = self.x_intercept(other);

        // NOTE: this will give the same value using other too
        // y = self.coefficient * ln(x) + self.intercept

        self.coefficient * x_int.ln() + self.intercept
    }
}
