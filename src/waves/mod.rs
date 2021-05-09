use crate::fp::list::List;

use crate::grid::Grid;
use crate::waveguide;

pub fn gaussian(grid_width: &Grid, position: f64, amplitude: f64, width: f64) -> List<waveguide::Phasor> {

    let position_normalized = position / grid_width.delta;

    let beam = grid_width.get_indexs().map(
        |x| {
            let r = x - position_normalized;
            
            amplitude*(- (r.powf(2.0) / width.powf(2.0)) ).exp()
        }
    ).collect();

    return beam_as_phasor(beam);
}

fn beam_as_phasor(l: List<f64>) -> List<waveguide::Phasor> {
    return l.into_iter().map(|x| waveguide::to_phasor(x)).collect();
}