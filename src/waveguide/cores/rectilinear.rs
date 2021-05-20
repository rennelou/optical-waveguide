use super::*;

pub struct Rectilinear<const D: usize> {
    pub shape: [usize;D],
    pub deltas: [f64;D],
    pub position: f64,
    n: f64,
    n0: f64,
    core_left: f64,
    core_right: f64
}

pub fn new_2d(dx: f64, xdelta: f64, dz: f64, zdelta: f64, n: f64, n0: f64, position: f64, core_width: f64) -> Rectilinear<2> {
    if position >= dx|| core_width >= dx {
        panic!("percent parameters need be less than 1");
    }

    let zsteps = (dz/zdelta).round() as usize;
    let xsteps = (dx/xdelta).round() as usize;
    let shape = [zsteps, xsteps];

    let deltas = [zdelta, xdelta];

    let core_left = position - (core_width/2.0);
    let core_right = position + (core_width/2.0);
    
    Rectilinear { shape, deltas, position, n, n0, core_left, core_right }
}

pub fn new_3d(dx: f64, xdelta: f64, dy: f64, ydelta: f64,dz: f64, zdelta: f64, n: f64, n0: f64, position: f64, core_width: f64) -> Rectilinear<3> {
    if position >= dx|| core_width >= dx {
        panic!("percent parameters need be less than 1");
    }

    let zdepht = (dz/zdelta).round() as usize;
    let ydepht = (dy/ydelta).round() as usize;
    let xdepht = (dx/xdelta).round() as usize;
    
    let shape = [zdepht, ydepht, xdepht];
    let deltas = [zdelta, ydelta, xdelta];

    let core_left = position - (core_width/2.0);
    let core_right = position + (core_width/2.0);
    
    Rectilinear { shape, deltas, position, n, n0, core_left, core_right }
}

impl<const D: usize> Core<D> for Rectilinear<D> {
    
    fn get_shape(&self) -> &[usize;D] {
        &self.shape
    }

    fn get_deltas(&self) -> &[f64;D] {
        &self.deltas
    }

    fn get_n(&self, position: &[usize], n0: f64) -> f64 {
        let new_shape = &position[1..];
        let new_deltas = &self.get_deltas()[1..];
        match self.get_dimension() {
            2 => {
                let x = new_shape[0] as f64 * new_deltas[0];
                if x > self.core_left && x < self.core_right {
                    self.n
                } else {
                    n0
                }
            },
            3 => {
                let y = new_shape[0] as f64 * new_deltas[0];
                let x = new_shape[1] as f64 * new_deltas[1];
                if y > self.core_left && y < self.core_right && x > self.core_left && x < self.core_right {
                    self.n
                } else {
                   n0
                }
            },
            _ => panic!("core must be 2 or 3 dimensions")
        }
    }

    fn get_n0(&self) -> f64 {
        return self.n0;
    }
}