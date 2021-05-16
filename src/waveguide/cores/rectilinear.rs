use super::*;

pub struct Rectilinear {
    pub shape: Vec<usize>,
    pub deltas: Vec<f64>,
    pub position: f64,
    n: f64,
    n0: f64,
    core_left: f64,
    core_right: f64
}

pub fn new_2d(dx: f64, xdelta: f64, dz: f64, zdelta: f64, n: f64, n0: f64, position: f64, core_width: f64) -> Rectilinear {
    if position >= dx|| core_width >= dx {
        panic!("percent parameters need be less than 1");
    }

    let zsteps = (dz/zdelta).round() as usize;
    let xsteps = (dx/xdelta).round() as usize;
    let shape = vec![zsteps, xsteps];

    let deltas = vec![zdelta, xdelta];

    let core_left = position - (core_width/2.0);
    let core_right = position + (core_width/2.0);
    
    Rectilinear { shape, deltas, position, n, n0, core_left, core_right }
}

impl Core for Rectilinear {
    
    fn get_shape(&self) -> &Vec<usize> {
        &self.shape
    }

    fn get_deltas(&self) -> &Vec<f64> {
        &self.deltas
    }

    fn get_n(&self, _: f64, y: f64, x: f64, n0: f64) -> f64 {
        match self.get_dimension() {
            2 => if x > self.core_left && x < self.core_right {
                    self.n
                } else {
                    n0
                },
            3 => if x > self.core_left && x < self.core_right && y > self.core_left && y < self.core_right {
                    self.n
                } else {
                   n0
                },
            _ => panic!("core must be 2 or 3 dimensions")
        }
    }

    fn get_n0(&self) -> f64 {
        return self.n0;
    }
}