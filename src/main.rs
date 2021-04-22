use rust_fdmbpm::waveguide::core_waveguide;
use rust_fdmbpm::waves;
use rust_fdmbpm::waveguide::slab;
use rust_fdmbpm::plotters;
use num::complex::Complex;
use core::f64::consts::PI;

fn main() {
    
    let dx = 260.0;
    let xdelta = dx/1024.0;
    
    let zdelta = 0.5;
    let dz = zdelta * 4000.0;

    let core_position = dx/2.0;
    let core_width = 20.0;

    let n0 = 3.0;
    let n = 3.3;

    let core = core_waveguide::rectilinear::new(dx, xdelta, dz, zdelta, n, core_position, core_width);
    
    let gaussian = waves::gaussian(&core, 12.0, 20.0);

    let w = slab::new(&core, n0, (2.0*PI)/1.55, 0.0, Complex::new(-10000.0, 0.0), Complex::new(-10000.0, 0.0));

    let es_2d = slab::fdmbpm(&w, gaussian);

    plotters::plot_waveguide_2d(core, es_2d, n0, 50);
}