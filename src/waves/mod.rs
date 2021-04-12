use crate::fp::list::List;
use crate::array::Array2d;
use core::f64::consts::PI;

pub fn gaussian(grid: &Array2d, position: f64, max_height: f64, sigma: f64) -> List<f64> {
        
    //let norm_max_height = max_height * grid.zdelta;
    let norm_sigma = sigma * grid.xdelta / max_height;

    let first_piece = 1.0 / ((2.0*PI*norm_sigma.powf(2.0)).sqrt());
    
    return grid.get_x_points().map(
        |x| {
            let norm_x = x / max_height;
            let norm_position = position / max_height;
            first_piece * (-( (norm_x-norm_position).powf(2.0) / (2.0*norm_sigma.powf(2.0)) )).exp()
        }
    ).collect();
}