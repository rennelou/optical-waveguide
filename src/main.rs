use rust_fdmbpm::waves;
use rust_fdmbpm::fp::list::List;
use rust_fdmbpm::waveguide::slab;
use rust_fdmbpm::plotters;
use num::complex::Complex;

fn main() {
    
    let dx = 10.0;
    let xdelta = 0.001;
    let dz = 20.0;
    let zdelta = 0.5;

    let w = slab::new(dx, xdelta, dz, zdelta, 1.0/1550.0, 3.4757, 3.4757, 0.0, Complex::new(1.0, 0.0), Complex::new(1.0, 0.0));
    
    let gaussian = waves::gaussian(w.get_x_points(), 5.0, 0.2);
    
    let es_2d = w.fdmbpm(f64_to_complex(gaussian));

    plotters::plot_waveguide_2d(es_2d);
}

fn f64_to_complex(l: List<f64>) -> List<Complex<f64>> {
    return l.into_iter().map(|x|Complex::new(x, 0.0)).collect();
}