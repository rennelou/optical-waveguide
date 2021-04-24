use rust_fdmbpm::waveguide::core_waveguide;
use rust_fdmbpm::waveguide::boundary_codition;
use rust_fdmbpm::waves;
use rust_fdmbpm::waveguide::slab;
use rust_fdmbpm::plotters;
use core::f64::consts::PI;

fn main() {
    
    let k0 = (2.0*PI)/1.55;

    let dx = 100.0 * k0;
    let xdelta = dx/1024.0;
    
    let zdelta = 0.5 * k0;
    let dz = zdelta * 200.0;

    let position = dx/2.0;
    let width = 10.0 * k0;

    let n0 = 3.0;
    let n = 3.3;

    let core = core_waveguide::rectilinear::new(dx, xdelta, dz, zdelta, n, n0, position, width);
    
    let gaussian = waves::gaussian(core.grid.get_x(), core.position, 4.5, 9.0*k0);

    let w = slab::new(&core, 1.0, 0.0);

    let es_2d = slab::fdmbpm(&w, gaussian, boundary_codition::dirichlet);

    plotters::plot_waveguide_2d(core, es_2d, n0, 50);
}