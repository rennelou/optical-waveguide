#[cfg(test)]
mod tests {
    use rust_fdmbpm::waveguide::cores;
	use rust_fdmbpm::waveguide::fdmbpm;
	use rust_fdmbpm::waveguide::boundary_codition;
	use rust_fdmbpm::waves;
	use core::f64::consts::PI;
	use ndarray::Array;
	use std::error::Error;

    #[test]
   	fn slab() -> Result<(), Box<dyn Error>> {
		let k0 = (2.0*PI)/1.55e-6_f64;

		let xdepht = 1024usize;
		let zdepht = 1000usize;

    	let dx = 260e-6 * k0;
    	let xdelta = dx/(xdepht as f64);
		
    	let zdelta = 0.5e-6 * k0;
    	let dz = zdelta * (zdepht as f64);

    	let position = dx/2.0;
    	let width = 35e-6 * k0;

		let shape = [xdepht];
		let deltas = [xdelta];
		let center = [position];

    	let n0 = 3.0;
    	let n = 3.3;

    	let core = cores::rectilinear::new_2d(dx, xdelta, dz, zdelta, n, n0, position, width);
		
    	let p = 200.0;
    	let eta = 120.0 * PI; // eta usa eps e mi do meio
    	let w = 10e-6 * k0;
    	let e0 = p*eta / (w.powf(2.0)*PI);
    	let gaussian = waves::gaussian(&shape, &deltas, &center, e0, w);

    	let e = fdmbpm::slab2d::run(&core, 1.0, 0.0, gaussian, boundary_codition::dirichlet);
		// para gerar seria so exportar e -- export::hdf5("example.h5", &e);

		let intensity = e.get_intensity();
    	let array = Array::from_shape_vec(e.shape().clone(), intensity)?;

    	let file = hdf5::File::open("slab.h5")?;
		let dir = file.group("dir")?;
		let values = dir.dataset("intensity")?;

		assert_eq!(values.read_dyn::<f64>()?, array);

		Ok(())
   	}
}