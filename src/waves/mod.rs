use crate::fp::list::List;
use crate::waveguide;

pub fn gaussian(grid_width: f64, delta: f64, position: f64, amplitude: f64, core_width: f64) -> List<waveguide::Phasor> {

    let steps = (grid_width / delta) as usize;
    let position_normalized = position / delta;

    let beam = (0..steps).map(|i| i as f64).map(
        |x| {
            let r = x - position_normalized;
            
            amplitude*(- (r.powf(2.0) / core_width.powf(2.0)) ).exp()
        }
    ).collect();

    return beam_as_phasor(beam);
}

fn beam_as_phasor(l: List<f64>) -> List<waveguide::Phasor> {
    return l.into_iter().map(|x| waveguide::to_phasor(x)).collect();
}