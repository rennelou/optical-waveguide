pub mod rectilinear;

pub trait Core {
    fn get_n(&self, x: f64, z: f64, n0: f64) -> f64;

    fn get_half_n(&self, x: f64, z: f64, n0: f64) -> f64;
} 