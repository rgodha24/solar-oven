use crate::Design;

impl Design {
    fn absorber_cost(&self) -> f64 {
        let area = self.asb();

        self.absorber.cost_per_m2(area)
    }

    fn inner_body_cost(&self) -> f64 {
        let area = self.asb();

        self.inner_body.cost_per_m2(area)
    }

    fn insulator_cost(&self) -> f64 {
        // inner_volume = (h + i_b_t) * (l_and_w + i_b_t * 2) ^ 2
        // outer_volume = (h + i_b_t + i_t) * (l_and_w + i_b_t * 2 + i_t * 2) ^ 2
        // insulator_volume = outer_volume - inner_volume

        let inner_volume = (self.h + self.inner_body.thickness())
            * (self.l_and_w + self.inner_body.thickness() * 2.).powi(2);
        let outer_volume = (self.h + self.inner_body.thickness() + self.insulator_thickness)
            * (self.l_and_w + self.inner_body.thickness() * 2. + self.insulator_thickness * 2.)
                .powi(2);

        let insulator_volume = outer_volume - inner_volume;

        self.insulator.cost_per_m3(insulator_volume)
    }

    fn outer_body_cost(&self) -> f64 {
        // top side window isn't made out of `outer_body` material
        // surface area - aw
        let h = self.h + self.inner_body.thickness() + self.insulator_thickness;
        let l = self.l_and_w
            + self.inner_body.thickness() * 2.
            + self.insulator_thickness * 2.
            + self.outer_body.thickness() * 2.;
        let w = l.clone();

        let surface_area = 2. * h * l + 2. * h * w + 2. * l * w;

        self.outer_body.cost_per_m2(surface_area - self.aw())
    }

    fn window_cost(&self) -> f64 {
        let area = self.aw();

        self.window.cost_per_m2(area)
    }

    fn reflector_cost(&self) -> f64 {
        let area = self
            .reflector_type
            .area(self.reflector_ml, self.l_and_w, self.reflector_count);

        self.reflectors.cost_per_m2(area)
    }

    pub fn total_cost(&self) -> f64 {
        self.absorber_cost()
            + self.inner_body_cost()
            + self.insulator_cost()
            + self.outer_body_cost()
            + self.window_cost()
            + self.reflector_cost()
    }
}
