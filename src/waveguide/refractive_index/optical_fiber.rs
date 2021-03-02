use super::*;

pub struct OpticalFiber {
    n: f64,
    n0: f64,
    core_left: f64,
    core_right: f64
}

pub fn new(n: f64, n0: f64, core_left: f64, core_right: f64) -> OpticalFiber {
    OpticalFiber { n, n0, core_left, core_right }
}

impl RefractiveIndex for OpticalFiber {
    fn get_n(&self, x: f64, _: f64) -> f64 {
        return {
            if x > self.core_left && x < self.core_right {
                self.n
            } else {
                self.n0
            }
        }
    }

    fn get_n0(&self) -> f64 {
        self.n0
    }

    fn get_half_n(&self, _: f64, _: f64) -> f64 {
        self.n0
    }
}

