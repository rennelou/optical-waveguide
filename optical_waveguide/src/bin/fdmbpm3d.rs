use optical_waveguide::fdmbpm::cores;
use optical_waveguide::fdmbpm::boundary_codition;
use optical_waveguide::fdmbpm::beam;
use optical_waveguide::fdmbpm::grid;
use optical_waveguide::fdmbpm::slab;
use optical_waveguide::export;

use core::f64::consts::PI;

fn main() -> Result<(), std::io::Error> {
	let k0 = (2.0*PI)/1.15;
	let xdepht = 100usize;
	let ydepht = 100usize;
    let dx = 40.0;
    let xdelta = dx/(xdepht as f64);
	let dy = 40.0;
	let ydelta = dy/(ydepht as f64);
	
	let dz = 200.0;
    let zdelta = 0.5;
    
    let position_x = dx/2.0;
	let position_y = dy/2.0;
    let width = 8.0;
	
	let center = [position_y, position_x];
    let n0 = 3.377;
    let n = 3.38;

	let grid = grid::new3(dx, xdelta, dy, ydelta, dz, zdelta);
    let core = cores::rectilinear::new_3d(n, n0, position_x, width);
	
    let w = 2.0;
    let beam = beam::gaussian(center, 1.0, w, k0, 0.0);

	let simulation = slab::new(grid.clone(), core.clone(), beam, boundary_codition::transparent); 
	let e = simulation.run();
	
	export::hdf5("fdmbpm3d.h5", &e, &grid, &core);
	
	Ok(())
}
   	