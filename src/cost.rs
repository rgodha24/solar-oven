use crate::design::Design;
use crate::materials::*;

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
    fn absorber_cost(&self) -> f64 {
        let area = self.asb();

        self.absorber.cost_per_m2(area)
    }

    fn inner_body_cost(&self) -> f64 {
        let area = self.asb();

        self.inner_body.cost_per_m2(area)
    }

    fn insulator_cost(&self) -> f64 {
        // inner_volume = (h + i_b_t) * (l_and_w + i_b_y) ^ 2
        // outer_volume = (h + i_b_t + i_t) * (l_and_w + i_b_t + i_t) ^ 2
        // insulator_volume = outer_volume - inner_volume

        let inner_volume = (self.h + self.inner_body_thickness)
            * (self.l_and_w + self.inner_body_thickness).powi(2);
        let outer_volume = (self.h + self.inner_body_thickness + self.insulator_thickness)
            * (self.l_and_w + self.inner_body_thickness + self.insulator_thickness).powi(2);

        let insulator_volume = outer_volume - inner_volume;

        self.insulator.cost_per_m3(insulator_volume)
    }

    fn outer_body_cost(&self) -> f64 {
        // surface area - l_and_w ^ 2
        let h = self.h + self.inner_body_thickness + self.insulator_thickness;
        let l = self.l_and_w + self.inner_body_thickness + self.insulator_thickness;
        let w = l.clone();

        let surface_area = 2. * h * l + 2. * h * w + l * w;

        self.outer_body.cost_per_m2(surface_area)
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
