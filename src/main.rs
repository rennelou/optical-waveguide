use core::f64::consts::PI;
use rust_fdmbpm::fp::list::List;
use rust_fdmbpm::waveguide::slab;
use num::complex::Complex;
use plotters::prelude::*;

fn main() {
    
    let w = slab::mock::get_waveguide_mock(10.0, 0.001, 20.0, 1.0, 1.0/1550.0, 3.4757, 3.4757, 0.2, Complex::new(0.1, 0.0), Complex::new(0.0, 0.0));
	
    let normal = gaussian(10000, 5.0, 0.5);
    let c = normal.iter().cloned().map(|x|Complex::new(x, 0.0)).collect();
    
    let fdmbpm = w.fdmbpm(c);

    let root_drawing_area = BitMapBackend::new("waveguide.png", (1024, 768))
        .into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .caption("Waveguide", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0.0..10.0, 0.0..20.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    for (z, l) in fdmbpm.iter().enumerate() {
        chart.draw_series(LineSeries::new(
            l.iter().enumerate().map(|(x,c)|{
                let (r, theta) = c.clone().to_polar();
                (x as f64, r * theta.cos() + z as f64)
            }),
            &RED
        )).unwrap();
    }
}

fn gaussian(size: i64, mean: f64, sigma: f64) -> List<f64> {
        
    let first_piece = 1.0 / (sigma * (2.0_f64*PI).sqrt());
    
    return (0..size).map(|i| i as f64).map(
        |x| first_piece * ( (-0.5)*((x-mean)/sigma).powf(2.0) ).exp()
    ).collect();
}