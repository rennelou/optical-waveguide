use crate::fp::{Matrix, matrix};
use crate::waveguide;

pub struct Gaussian<const D: usize> {
    pub center: [f64;D],
    pub amplitude: f64, 
    pub width: f64,
    pub k: f64,
    pub alpha: f64
}

pub fn new<const D: usize>(center: [f64;D], amplitude: f64, width: f64, k: f64, alpha: f64) -> Gaussian<D> {
    Gaussian {center, amplitude, width, k, alpha}
}

// DEVERIA RECEBER A GRID E NÃO OS VALORES SOLTOS
pub fn input<const D: usize>(shape: &[usize;D], deltas: &[f64;D], center: &[f64;D], amplitude: f64, width: f64) -> Matrix<waveguide::Phasor> {

    let center_normalized: Vec<_> = center.iter().zip(deltas.iter()).map(
        |(&p, &d)| p/d
    ).collect();

    let values = (0..shape.iter().product()).map(|id| {
        let position = matrix::id_to_position(id, shape);
        
        let v = position.iter().zip(center_normalized.iter()).map(
            |(&p, p0)| (p as f64) - p0
        );
        let v = v.zip(deltas.iter()).map(|(p, &d)|
            p * d
        );

        let r = v.map(|x| x.powf(2.0)).sum::<f64>().sqrt();
        let e = amplitude*(- (r.powf(2.0) / width.powf(2.0)) ).exp();
            
        waveguide::to_phasor(e)

    }).collect();

    matrix::new(values, shape)
}