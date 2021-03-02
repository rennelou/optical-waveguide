pub mod optical_fiber;
pub trait RefractiveIndex {
    fn get_n(&self, x: f64, z: f64) -> f64;

    fn get_n0(&self) -> f64;

    fn get_half_n(&self, x: f64, z: f64) -> f64;
} 