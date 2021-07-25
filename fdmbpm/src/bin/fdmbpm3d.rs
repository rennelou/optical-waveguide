use rust_fdmbpm::waveguide::cores;
use rust_fdmbpm::waveguide::boundary_codition;
use rust_fdmbpm::waves;
use rust_fdmbpm::waveguide::fdmbpm;
use rust_fdmbpm::export;

use core::f64::consts::PI;

fn main() -> Result<(), std::io::Error> {
	let k0 = (2.0*PI)/1.15;
	let xdepht = 100usize;
	let ydepht = 50usize;
    let dx = 40.0;
    let xdelta = dx/(xdepht as f64);
	let dy = 40.0;
	let ydelta = dy/(ydepht as f64);
	
	let dz = 200.0;
    let zdelta = 0.5;
    
    let position_x = dx/2.0;
	let position_y = dy/2.0;
    let width = 8.0;
	
	let shape = [ydepht, xdepht];
	let deltas = [ydelta, xdelta];
	let center = [position_y, position_x];
    let n0 = 3.377;
    let n = 3.38;
    let core = cores::rectilinear::new_3d(dx, xdelta, dy, ydelta, dz, zdelta, n, n0, position_x, width);
	
    let w = 2.0;
    let gaussian = waves::gaussian(&shape, &deltas, &center, 1.0, w);
    let e = fdmbpm::slab3d::run(&core, k0, 0.0, gaussian, boundary_codition::transparent);
	
	export::hdf5("fdmbpm3d.h5", &e, &core);
	
	Ok(())
}
   	