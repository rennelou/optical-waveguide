use itertools::izip;
use crate::fp::{Matrix, matrix};
use crate::fdmbpm;

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

pub fn input<const D: usize>(shape: &[usize;D], deltas: &[f64;D], center: &[f64;D], amplitude: f64, width: f64) -> Matrix<fdmbpm::Phasor> {

    let center_normalized: Vec<_> = izip!(center.iter(), deltas.iter()).map(
        |(&p, &d)| p/d
    ).collect();

    let values = matrix::dephts_cartesian_product(shape.to_vec()).into_iter().map(
        |position| {
            let v = izip!(&position, &center_normalized).map(
                |(&p, &p0)| (p as f64) - p0
            );
            let v = izip!(v, deltas).map(
                |(p, &d)| p * d
            );
    
            let r = v.map(|x| x.powf(2.0)).sum::<f64>().sqrt();
            let e = amplitude*(- (r.powf(2.0) / width.powf(2.0)) ).exp();
                
            fdmbpm::to_phasor(e)
        }
    ).collect();

    matrix::new_from_raw(values, shape)
}