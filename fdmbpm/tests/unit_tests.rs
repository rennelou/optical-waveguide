#[cfg(test)]
mod tests {
    use rust_fdmbpm::tools;
    use rust_fdmbpm::waveguide::cores;
	use rust_fdmbpm::waveguide::fdmbpm;
	use rust_fdmbpm::waveguide::boundary_codition;
	use rust_fdmbpm::waves;
	use core::f64::consts::PI;
	use std::error::Error;

    #[test]
   	fn core_8_gaussian_4() -> Result<(), Box<dyn Error>> {
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
		let beam = waves::new(center, 1.0, w, k0, 0.0);
		
		let e = fdmbpm::slab2d::run(&core, beam, boundary_codition::transparent);

    	let file = hdf5::File::open("tests/datas/core_8_gaussian_4.h5")?;
		let reference = file.dataset("intensity").unwrap();
		
		let diffs = tools::areas_diff(
			tools::normalize(tools::dataset_to_matrix(reference)), 
			tools::normalize(e.get_intensity())
		);
		
		assert!(diffs.into_iter().all(|x| x <= 0.011)); // erro de ate 1.1%

		Ok(())
   	}

	#[test]
	fn core_8_gaussian_8() -> Result<(), Box<dyn Error>> {
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
		
    	let w = 4.0_f64;
		let beam = waves::new(center, 1.0, w, k0, 0.0);
		
		let e = fdmbpm::slab2d::run(&core, beam, boundary_codition::transparent);

    	let file = hdf5::File::open("tests/datas/core_8_gaussian_8.h5")?;
		let reference = file.dataset("intensity").unwrap();
		
		let diffs = tools::areas_diff(
			tools::normalize(tools::dataset_to_matrix(reference)), 
			tools::normalize(e.get_intensity())
		);
		
		assert!(diffs.into_iter().all(|x| x <= 0.011)); // erro de ate 1.1%

		Ok(())
   	}

	#[test]
	fn slab3d() -> Result<(), Box<dyn Error>> {
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

	   let core = cores::rectilinear::new_3d(dx, xdelta, dy, ydelta, dz, zdelta, n, n0, position_x, width);
	   
	   let w = 2.0;
	   let beam = waves::new(center, 1.0, w, k0, 0.0);

	   let e = fdmbpm::slab3d::run(&core, beam, boundary_codition::transparent);

	   let file = hdf5::File::open("tests/datas/slab3d.h5")?;
	   let reference = file.dataset("intensity").unwrap();
	   
	   let diffs = tools::areas_diff(
		   tools::normalize(tools::dataset_to_matrix(reference)), 
		   tools::normalize(e.get_intensity())
	   );
	   
	   assert!(diffs.into_iter().all(|x| x <= 0.011)); // erro de ate 1.1%
	   
	   Ok(())
	  }
}