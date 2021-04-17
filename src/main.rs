use rust_fdmbpm::array;
use rust_fdmbpm::waveguide::core_waveguide;
use rust_fdmbpm::waves;
use rust_fdmbpm::fp::list::List;
use rust_fdmbpm::waveguide::slab;
use rust_fdmbpm::plotters;
use num::complex::Complex;
use core::f64::consts::PI;

fn main() {
    
    let dx = 260.0;
    let xdelta = dx/1024.0;
    
    let zdelta = 0.5;
    let dz = zdelta * 200.0;

    let core_position = dx/2.0;
    let core_width = 30.0;

    let n0 = 3.0;

    let grid = array::Array2d::new(dx, xdelta, dz, zdelta);
	let r = core_waveguide::rectilinear::new(3.3, &grid, core_position, core_width);
    let w = slab::new(&grid, &r, n0, (2.0*PI)/1.55, 0.0, Complex::new(-10000.0, 0.0), Complex::new(-10000.0, 0.0));

    let gaussian = waves::gaussian(&grid, core_position, 50.0, 20.0);

    let es_2d = w.fdmbpm(f64_to_complex(gaussian));

    plotters::plot_waveguide_2d(grid, es_2d, r, n0, 50);
}

fn f64_to_complex(l: List<f64>) -> List<Complex<f64>> {
    return l.into_iter().map(|x|Complex::new(x, 0.0)).collect();
}