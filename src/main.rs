use rust_fdmbpm::waves::gaussian;
use rust_fdmbpm::fp::list::List;
use rust_fdmbpm::waveguide::slab;
use num::complex::Complex;
use plotters::prelude::*;

fn main() {
    let dx = 10.0;
    let xdelta = 0.001;
    let dz = 20.0;
    let zdelta = 0.5;

    let w = slab::new(dx, xdelta, dz, zdelta, 1.0/1550.0, 3.4757, 3.4757, 0.0, Complex::new(1.0, 0.0), Complex::new(1.0, 0.0));
    
    let normal = gaussian(w.get_x_points(), 5.0, 0.2);
    
    let es_2d = w.fdmbpm(f64_to_complex(normal));

    let root_drawing_area = BitMapBackend::new("waveguide.png", (1024, 768))
        .into_drawing_area();
    
    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .caption("Waveguide", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0.0..dx, 0.0..dz)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    for line in es_2d.get_points() {
        chart.draw_series(LineSeries::new(
            line.iter().map(|p| (p.x, p.z + p.eletric_field)),
            &RED
        )).unwrap();
    }
}

fn f64_to_complex(l: List<f64>) -> List<Complex<f64>> {
    return l.iter().copied().map(|x|Complex::new(x, 0.0)).collect();
}