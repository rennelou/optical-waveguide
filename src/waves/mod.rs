use crate::fp::list::List;
use crate::array::Array;

pub fn gaussian(grid: &Array, position: f64, amplitude: f64, width: f64) -> List<f64> {
    
    let position_normalized = position / grid.delta;

    return grid.get_indexs().map(
        |x| {
            let r = x - position_normalized;
            
            amplitude*(- (r.powf(2.0) / width.powf(2.0)) ).exp()
        }
    ).collect();
}