use crate::array::Array2d;
use super::*;

pub struct Rectilinear {
    pub grid: Array2d,
    position: f64,
    n: f64,
    core_left: f64,
    core_right: f64
}

pub fn new(dx: f64, xdelta: f64, dz: f64, zdelta: f64, n: f64, position: f64, core_width: f64) -> Rectilinear {
    let grid = Array2d::new(dx, xdelta, dz, zdelta);

    if position >= dx|| core_width >= dx {
        panic!("percent parameters need be less than 1");
    }

    let core_left = position - (core_width/2.0);
    let core_right = position + (core_width/2.0);
    
    Rectilinear { grid: grid, position, n, core_left, core_right }
}

impl Core for Rectilinear {
    fn get_grid(&self) -> &Array2d {
        return &self.grid;
    }

    fn get_position(&self) -> f64 {
        return self.position;
    }
    
    fn get_n(&self, x: f64, _: f64, n0: f64) -> f64 {
        return {
            if x > self.core_left && x < self.core_right {
                self.n
            } else {
                n0
            }
        }
    }

    fn get_half_n(&self, x: f64, z: f64, n0: f64) -> f64 {
        return self.get_n(x, z, n0);
    }
}

