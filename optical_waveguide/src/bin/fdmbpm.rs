use optical_waveguide::fdmbpm::cores;
use optical_waveguide::fdmbpm::boundary_codition;
use optical_waveguide::fdmbpm::beam;
use optical_waveguide::fdmbpm::slab;
use optical_waveguide::export;

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

	let center = [position];
    
	let n0 = 3.377;
    let n = 3.38;

    let core = cores::rectilinear::new_2d(dx, xdelta, dz, zdelta, n, n0, position, width);
	
    let w = 2.0_f64;
	let beam = beam::gaussian(center, 1.0, w, k0, 0.0);
	let simulation = slab::new(core.clone(), beam, boundary_codition::transparent); 
	let e = simulation.run();

    export::hdf5("main.h5", &e, &core);

    Ok(())
}