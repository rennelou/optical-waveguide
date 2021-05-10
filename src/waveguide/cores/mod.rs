pub mod rectilinear;

pub trait Core<const N: usize> {
    fn get_dimension() -> usize {
        N
    }

    fn get_shape(&self) -> [usize;N];

    fn get_deltas(&self) -> [f64;N];

    fn get_n(&self, x: f64, z: f64, n0: f64) -> f64;

    fn get_half_n(&self, x: f64, z: f64, n0: f64) -> f64;

    fn get_n0(&self) -> f64;
} 