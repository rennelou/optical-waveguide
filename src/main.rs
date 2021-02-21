use core::f64::consts::PI;
use rust_fdmbpm::fp::list::List;
use rust_fdmbpm::waveguide::slab;
use num::complex::Complex;
use plotters::prelude::*;

fn main() {
    
    let w = slab::new(10.0, 0.001, 20.0, 0.5, 1.0/1550.0, 3.4757, 3.4757, 0.0, Complex::new(1.0, 0.0), Complex::new(0.1, 0.0));
	
    let x_points = w.get_x_points();
    let z_points = w.get_z_points();
    let normal = gaussian(&x_points, 5.0, 0.2);
    
    let fdmbpm = w.fdmbpm(f64_to_complex(normal));

    let root_drawing_area = BitMapBackend::new("waveguide.png", (1024, 768))
        .into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .caption("Waveguide", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0.0..10.0, 0.0..22.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    for (offset, l) in z_points.iter().cloned().zip(fdmbpm) {
        chart.draw_series(LineSeries::new(
            x_points.iter().cloned().zip(
                   l.iter().map(|c|{
                        let (r, theta) = c.clone().to_polar();
                        offset + (r * theta.cos())
                    }
                )
            ),
            &RED
        )).unwrap();
    }
}

fn gaussian(points: &List<f64>, mean: f64, sigma: f64) -> List<f64> {
        
    let first_piece = 1.0 / (sigma * (2.0_f64*PI).sqrt());
    
    return points.iter().map(
        |x| first_piece * ( (-0.5)*((x-mean)/sigma).powf(2.0) ).exp()
    ).collect();
}

fn f64_to_complex(l: List<f64>) -> List<Complex<f64>> {
    return l.iter().cloned().map(|x|Complex::new(x, 0.0)).collect();
}