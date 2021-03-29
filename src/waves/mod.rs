use crate::fp::list::List;
use crate::array::Array2d;
use core::f64::consts::PI;

pub fn gaussian(grid: &Array2d, position: f64, max_height: f64, sigma: f64) -> List<f64> {
        
    let norm_max_height = max_height * grid.zdelta;
    let norm_sigma = sigma * grid.xdelta;

    let first_piece = norm_max_height / ((2.0*PI*norm_sigma.powf(2.0)).sqrt());
    
    return grid.get_x_points().map(
        |x| first_piece * (-( (x-position).powf(2.0) / (2.0*norm_sigma.powf(2.0)) )).exp()
    ).collect();
}