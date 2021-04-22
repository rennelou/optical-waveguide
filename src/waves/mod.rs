use crate::fp::list::List;

use crate::array::Array;
use num::complex::Complex;

pub fn gaussian(grid_width: &Array, position: f64, amplitude: f64, width: f64) -> List<Complex<f64>> {

    let position_normalized = position / grid_width.delta;

    let beam = grid_width.get_indexs().map(
        |x| {
            let r = x - position_normalized;
            
            amplitude*(- (r.powf(2.0) / width.powf(2.0)) ).exp()
        }
    ).collect();

    return f64_to_complex(beam);
}

fn f64_to_complex(l: List<f64>) -> List<Complex<f64>> {
    return l.into_iter().map(|x|Complex::new(x, 0.0)).collect();
}