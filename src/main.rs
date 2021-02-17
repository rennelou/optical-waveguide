use rust_fdmbpm::waveguide::slab;
use num::complex::Complex;
use plotters::prelude::*;

fn main() {
    let w = slab::mock::get_waveguide_mock(10.0, 2.0, 10.0, 1.0, 1.0/1550.0, 3.4757, 1.0, 0.2, Complex::new(1.0, 0.0), Complex::new(1.0, 0.0));
	let fdmbpm = w.fdmbpm(slab::mock::get_ones(5));

    let root_drawing_area = BitMapBackend::new("0.1.png", (1024, 768))
        .into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .build_cartesian_2d(0.0..10.0, 0.0..1.0)
        .unwrap();

    for (i,l) in fdmbpm.iter().enumerate() {
        let offset = i as f64 / 10.0;
        chart.draw_series(LineSeries::new(
            l.iter().enumerate().map(|(i,x)| (i as f64, x.norm()+offset)),
            &RED
        )).unwrap();
    }
}