use rust_fdmbpm::waveguide::cores;
use rust_fdmbpm::waveguide::boundary_codition;
use rust_fdmbpm::waves;
use rust_fdmbpm::waveguide::fdmbpm;
use rust_fdmbpm::export;

use core::f64::consts::PI;

fn main() -> Result<(), std::io::Error> {
    
    let k0 = 2.0*PI/1.15;
    
	let xdepht = 2000usize;
	let dx = 40.0;
    let xdelta = dx/(xdepht as f64);
	
    let dz = 750.0;
    let zdelta = 0.5;
    
	let position = dx/2.0;
    let width = 8.0;
	
	let shape = [xdepht];
	let deltas = [xdelta];
	let center = [position];
    
	let n0 = 3.377;
    let n = 3.38;

    let core = cores::rectilinear::new_2d(dx, xdelta, dz, zdelta, n, n0, position, width);
	
    let w = 2.0_f64;
	let gaussian = waves::gaussian(&shape, &deltas, &center, 1.0, w);
    
	let e = fdmbpm::slab2d::run(&core, k0, 0.0, gaussian, boundary_codition::transparent);
    export::hdf5("main.h5", &e, &core);

    Ok(())
}