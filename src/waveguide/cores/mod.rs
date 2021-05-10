pub mod rectilinear;

pub trait Core<const N: usize> {
    fn get_dimension() -> usize {
        N
    }

    fn get_shape(&self) -> [usize;N];

    fn get_deltas(&self) -> [f64;N];

    fn get_n(&self, z: f64, y: f64, x: f64, n0: f64) -> f64;

    fn get_half_n(&self, z: f64, y: f64, x: f64, n0: f64) -> f64 {
        let zdelta = self.get_deltas()[0];
        
        (self.get_n(z, y, x, n0) + self.get_n(z+zdelta, y, x, n0))/2.0
    }

    fn get_n0(&self) -> f64;
} 