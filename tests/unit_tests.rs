#[cfg(test)]
mod tests {
    use rust_fdmbpm::waveguide::core_waveguide;
	use rust_fdmbpm::waveguide::slab;
	use rust_fdmbpm::waveguide::boundary_codition;
	use rust_fdmbpm::waves;
	use core::f64::consts::PI;
	use ndarray::Array;

    #[test]
   	fn slab() {
		let k0 = (2.0*PI)/1.55e-6_f64;

    	let dx = 260e-6 * k0;
    	let xdelta = dx/1024.0;
		
    	let zdelta = 0.5e-6 * k0;
    	let dz = zdelta * 1000.0;

    	let position = dx/2.0;
    	let width = 35e-6 * k0;

    	let n0 = 3.0;
    	let n = 3.3;

    	let core = core_waveguide::rectilinear::new(dx, xdelta, dz, zdelta, n, n0, position, width);
		
    	let p = 200.0;
    	let eta = 120.0 * PI; // eta usa eps e mi do meio
    	let w = 10e-6 * k0;
    	let e0 = p*eta / (w.powf(2.0)*PI);
    	let gaussian = waves::gaussian(core.grid.get_x(), core.position, e0, w);

    	let es_2d = slab::fdmbpm(&core, 1.0, 0.0, gaussian, boundary_codition::dirichlet);
    	let intensity = es_2d.get_intensity();

    	let array = Array::from_shape_vec(intensity.shape, intensity.values).unwrap();

    	let file = hdf5::File::open("slab.h5").unwrap();
		let dir = file.group("dir").unwrap();
		let values = dir.dataset("intensity").unwrap();

		assert_eq!(values.read_2d::<f64>().unwrap(), array);
   	}
}