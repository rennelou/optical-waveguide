use crate::array::Array2d;
use super::*;

pub struct Rectilinear {
    n: f64,
    core_left: f64,
    core_right: f64
}

const X: usize = 0;

pub fn new(grid: &Array2d, n: f64, position: f64, core_width: f64) -> Rectilinear {
   let dx = grid.get(X).d;

    if position >= dx|| core_width >= dx {
        panic!("percent parameters need be less than 1");
    }

    let core_left = position - (core_width/2.0);
    let core_right = position + (core_width/2.0);
    
    Rectilinear { n, core_left, core_right }
}

impl Core for Rectilinear {
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
