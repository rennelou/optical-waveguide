pub mod waveguide {
	
	use num::complex::Complex;

	const MINIMALSTEP: u32 = 5;

	pub struct Slab {
		xsteps: i64,
		zsteps: i64,

		xdelta: f64,

		kright: Complex<f64>,
		kleft:  Complex<f64>,

		s: Vec<Vec<Complex<f64>>>,
		q: Vec<Vec<Complex<f64>>>,
	}

	pub struct Abc {
		a: Complex<f64>,
		b: Complex<f64>,
		c: Complex<f64>,
	}

	impl Slab {
	
	    pub fn new(dx: f64, xdelta: f64, dz: f64, zdelta: f64,
	    	k: f64, n: f64, n0: f64, alpha: f64, kleft: Complex<f64>, kright: Complex<f64>) -> Slab {

	    	let xsteps = (dx / xdelta).round() as i64;
	    	let zsteps = (dz / zdelta).round() as i64;
			
			let guiding_space = |_, _| Complex::new(k.sqrt()*xdelta.sqrt()*(n.sqrt()-n0.sqrt()), 0.0);
	    	let free_space = |_, _| Complex::new(0.0, 4.0*k*n0*xdelta.sqrt()/zdelta);
	    	let loss = |_, _| Complex::new(0.0, 2.0*k*n0*xdelta.sqrt()*alpha);

			let s = (0..zsteps).map(
				|i| (0..xsteps).map(
					// okamoto 7.98
					|j| Complex::new(2.0, 0.0)-guiding_space(i, j)+free_space(i, j)+loss(i, j)
				
				).collect()
			).collect();

			let q = (0..zsteps).map(
				|i| (0..xsteps).map(
					// okamoto 7.99
					|j| Complex::new(-2.0, 0.0)+guiding_space(i, j)+free_space(i, j)-loss(i, j)
				
				).collect()
			).collect();

	    	return Slab{
	    		xsteps: xsteps,
	    		zsteps: zsteps,
	    		xdelta: xdelta,
	    		kright: kright,
	    		kleft:  kleft,
	    		s:      s,
	    		q:      q,
	    	}
	    }
	}
}	