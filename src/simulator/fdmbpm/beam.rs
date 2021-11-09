use super::*;
use crate::functional_types::{Matrix, matrix};
use fdmbpm;
use itertools::izip;

pub struct Gaussian<const D: usize> {
    pub center: [f64;D],
    pub amplitude: f64, 
    pub width: f64,
    pub k: f64,
    pub alpha: f64
}

pub fn gaussian<const D: usize>(center: [f64;D], amplitude: f64, width: f64, k: f64, alpha: f64) -> Gaussian<D> {
    Gaussian {center, amplitude, width, k, alpha}
}

impl<const D: usize> Gaussian<D> {
    pub fn input(&self, shape: &[usize;D], deltas: &[f64;D]) -> Matrix<fdmbpm::Phasor> {

        let center_normalized: Vec<_> = izip!(self.center.iter(), deltas.iter()).map(
            |(&p, &d)| p/d
        ).collect();
    
        let values = matrix::cartesian_product_of_shape(shape.to_vec()).map(
            |position| {
                let v = izip!(&position, &center_normalized).map(
                    |(&p, &p0)| (p as f64) - p0
                );
                let v = izip!(v, deltas).map(
                    |(p, &d)| p * d
                );
        
                let r = v.map(|x| x.powf(2.0)).sum::<f64>().sqrt();
                let e = self.amplitude*(- (r.powf(2.0) / self.width.powf(2.0)) ).exp();
                    
                fdmbpm::to_phasor(e)
            }
        ).collect();
    
        matrix::new(values, shape)
    }
} 