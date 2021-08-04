#[derive(Clone)]
pub struct Grid<const D: usize> {
    shape: [usize;D],
    deltas: [f64;D]
}

pub fn new2(dx: f64, xdelta: f64, dz: f64, zdelta: f64) -> Grid<2> {
    let zsteps = (dz/zdelta).round() as usize;
    let xsteps = (dx/xdelta).round() as usize;
    let shape = [zsteps, xsteps];

    let deltas = [zdelta, xdelta];
    
    Grid { shape, deltas }
}

pub fn new3(dx: f64, xdelta: f64, dy: f64, ydelta: f64,dz: f64, zdelta: f64) -> Grid<3> {
    let zdepht = (dz/zdelta).round() as usize;
    let ydepht = (dy/ydelta).round() as usize;
    let xdepht = (dx/xdelta).round() as usize;
    
    let shape = [zdepht, ydepht, xdepht];
    let deltas = [zdelta, ydelta, xdelta];

    Grid { shape, deltas }
}

impl<const D: usize> Grid<D> {
    pub fn get_shape(&self) -> &[usize;D] {
        &self.shape
    }

    pub fn get_deltas(&self) -> &[f64;D] {
        &self.deltas
    }
}
