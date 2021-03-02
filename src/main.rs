use rust_fdmbpm::waveguide::geometry;
use rust_fdmbpm::waveguide::refractive_index;
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

    let g = geometry::new(dx, xdelta, dz, zdelta);
	let r = refractive_index::optical_fiber::new(3.4757, 1.0, 4.5, 7.5);
    let w = slab::new(&g, 1.0/1550.0, r, 0.0, Complex::new(1.0, 0.0), Complex::new(1.0, 0.0));
    
    let gaussian = waves::gaussian(g.get_x_points(), 5.0, 0.2);
    
    let es_2d = w.fdmbpm(f64_to_complex(gaussian));

    plotters::plot_waveguide_2d(es_2d, g);
}

fn f64_to_complex(l: List<f64>) -> List<Complex<f64>> {
    return l.into_iter().map(|x|Complex::new(x, 0.0)).collect();
}