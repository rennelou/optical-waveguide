use rust_fdmbpm::waveguide::core_waveguide;
use rust_fdmbpm::waveguide::boundary_codition;
use rust_fdmbpm::waves;
use rust_fdmbpm::waveguide::slab;
use rust_fdmbpm::plotters;
use core::f64::consts::PI;

fn main() {
    
    let k0 = (2.0*PI)/1.55e-6_f64;

    let dx = 260e-6_f64 * k0;
    let xdelta = dx/1024.0;
    
    let zdelta = 0.5e-6_f64 * k0;
    let dz = zdelta * 1000.0;

    let position = dx/2.0;
    let width = 35e-6_f64 * k0;

    let n0 = 3.0;
    let n = 3.3;

    let p = 30e-6_f64 * k0;
    let eta = 120.0*PI;

    let core = core_waveguide::rectilinear::new(dx, xdelta, dz, zdelta, n, n0, position, width);
    
    let w = 10e-6_f64*k0;
    let e0 = p*eta / (w.powf(2.0)*PI);
    let gaussian = waves::gaussian(core.grid.get_x(), core.position, e0, w);

    let w = slab::new(&core, 1.0, 0.0);

    let es_2d = slab::fdmbpm(&w, gaussian, boundary_codition::dirichlet);

    plotters::plot_waveguide_2d(core, es_2d, n0, 50);
}