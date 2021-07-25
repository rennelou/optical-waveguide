use crate::fp::{Matrix, matrix};
use crate::waveguide;

// DEVERIA RECEBER A GRID E NÃO OS VALORES SOLTOS
pub fn gaussian<const D: usize>(shape: &[usize;D], deltas: &[f64;D], center: &[f64;D], amplitude: f64, width: f64) -> Matrix<waveguide::Phasor,D> {

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

    matrix::new(values, &shape)
}