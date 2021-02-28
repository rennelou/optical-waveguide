use crate::fp::list::List;
use core::f64::consts::PI;

pub fn gaussian(points: List<f64>, mean: f64, sigma: f64) -> List<f64> {
        
    let first_piece = 1.0 / (sigma * (2.0_f64*PI).sqrt());
    
    return points.iter().map(
        |x| first_piece * ( (-0.5)*((x-mean)/sigma).powf(2.0) ).exp()
    ).collect();
}