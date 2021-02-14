pub mod waveguide {
	
	use num::complex::Complex;

	const MINIMALSTEP: i64 = 5;

	pub struct Slab {
		xsteps: i64,
		zsteps: i64,

		xdelta: f64,

		kright: Complex<f64>,
		kleft:  Complex<f64>,

		s: Vec<Vec<Complex<f64>>>,
		q: Vec<Vec<Complex<f64>>>,
	}

	struct Abc {
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

		fn get_abcs(&self, z: usize) -> Vec<Abc> {
			let mut result: Vec<Abc> = Vec::new();
			
			if self.xsteps >= MINIMALSTEP {
				result.push(Abc {
					// okamoto 7.108a
					a: Complex::new(0.0, 0.0), 
					b: self.s[z][1] - self.left_boundary(z), 
					c: Complex::new(1.0, 0.0)
				});

				result.append(&mut (2..self.xsteps-2).map(
					// okamoto 7.108b
					|i| Abc {
						a: Complex::new(1.0, 0.0), 
						b: self.s[z][i as usize], 
						c: Complex::new(1.0, 0.0)
					}
				).collect());

				result.push(Abc {
					/// okamoto 7.108c
					a: Complex::new(1.0, 0.0), 
					b: self.s[z][(self.xsteps - 2) as usize] - self.right_boundary(z), 
					c: Complex::new(0.0, 0.0)
				});
			}

			return result;
		}

		fn right_boundary(&self, _z: usize) -> Complex<f64> {
			return ( 
				Complex::new(0.0, -1.0)*self.kright*Complex::new(self.xdelta, 0.0) 
			).exp();
		}

		fn left_boundary(&self, _z: usize) -> Complex<f64> {
			return ( 
				Complex::new(0.0, -1.0)*self.kleft*Complex::new(self.xdelta, 0.0) 
			).exp();
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;
    	use num::complex::Complex;

    	#[test]
    	fn assert_abcs_sizes() {
    	    for i in 1..10 {
    	        let w = get_waveguide_mock(100.0, i as f64, 2.0, 1.0, 1.0/1550.0, 3.4757, 1.0, 0.2, Complex::new(0.0, 0.0), Complex::new(0.0, 0.0));
				let got = w.get_abcs(0);

				assert_eq!(got.len(), (w.xsteps-2) as usize );
    	    }
    	}

    	fn get_waveguide_mock(dx: f64, xdelta: f64, dz: f64, zdelta: f64,
    		k: f64, n: f64, n0: f64, alpha: f64, kleft: Complex<f64>, kright: Complex<f64>) -> Slab {
			
    	    return Slab::new(dx, xdelta, dz, zdelta, k, n, n0, alpha, kleft, kright);
    	}
	}
}