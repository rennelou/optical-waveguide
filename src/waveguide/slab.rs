use super::*;
use fp::list;
use fp::list::List;

pub struct Slab {
	xsteps: i64,
	zsteps: i64,
	xdelta: f64,
	kright: Complex<f64>,
	kleft:  Complex<f64>,
	s: List<List<Complex<f64>>>,
	q: List<List<Complex<f64>>>,
}

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

impl Slab {

	pub fn fdmbpm(&self, e_input: List<Complex<f64>>) -> List<List<Complex<f64>>> {
		
		return (1..self.zsteps).fold(vec![e_input], |result, i| {
			
			let index = i as usize;

			let last_es = fp::unwrap_or_default(fp::last(&result), list::empty());
			let q = &self.q[index-1];
			
			let ds = get_ds(&last_es, q);
			let abcs = self.get_abcs(index);

			let new_es = self.insert_boundary_values(
				index, 
				get_recurrence_form(get_alphas_betas(abcs, ds)
				)
			);

			return list::push(result, new_es);
		})
	}

	fn get_abcs(&self, z: usize) -> List<Abc> {
		if self.xsteps >= MINIMALSTEP {
			
			let head = list::new(Abc {
				// okamoto 7.108a
				a: zero(), b: self.s[z][1] - self.left_boundary(z), c: one()
			});
			
			let body: Vec<_> = (2..self.xsteps-2).map(
				// okamoto 7.108b
				|i| Abc { a: one(), b: self.s[z][i as usize], c: one() }

			).collect();
			
			let last = list::new(Abc {
				/// okamoto 7.108c
				a: one(), 
				b: self.s[z][(self.xsteps - 2) as usize] - self.right_boundary(z), 
				c: zero()
			});

			return list::concat(list::concat(head,body),last);
		}
		return list::empty();
	}

	fn insert_boundary_values(&self, z: usize, es: List<Complex<f64>>) -> List<Complex<f64>>{
		
		let head = list::new({
			let es_head = fp::unwrap_or_default(fp::head(&es), one());

			es_head*self.left_boundary(z) // okamoto 7.106

		});

		let last = list::new({
			let es_last = fp::unwrap_or_default(fp::last(&es), one());

			es_last*self.right_boundary(z) // okamoto 7.105

		});
		
		return list::concat(list::concat(head, es),last);
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
   	use super::mock;
   	
	#[test]
   	fn assert_abcs_sizes() {
   	    for i in 1..10 {
   	        let w = mock::get_waveguide_mock(100.0, i as f64, 2.0, 1.0, 1.0/1550.0, 3.4757, 1.0, 0.2, zero(), zero());
			let got = w.get_abcs(0);
			assert_eq!(got.len(), (w.xsteps-2) as usize );
   	    }
   	}
	
	#[test]
   	fn assert_alpha_betas() {
   	    let w = mock::get_waveguide_mock(100.0, 10.0, 2.0, 1.0, 1.0/1550.0, 3.4757, 1.0, 0.2, zero(), zero());
		let got = get_alphas_betas(w.get_abcs(0), mock::get_zeros(10));
		println!("{:?}", got);
   	}
}

pub mod mock {
	use super::*;
   	use num::complex::Complex;

	pub fn get_zeros(i: i32) -> List<Complex<f64>> {
		return (0..i).map(|_| zero()).collect();
	}

	pub fn get_ones(i: i32) -> List<Complex<f64>> {
		return (0..i).map(|_| Complex::new(1.0, 0.0) ).collect();
	}
   	
	pub fn get_waveguide_mock(dx: f64, xdelta: f64, dz: f64, zdelta: f64,
   		k: f64, n: f64, n0: f64, alpha: f64, kleft: Complex<f64>, kright: Complex<f64>) -> Slab {
		
   	    return slab::new(dx, xdelta, dz, zdelta, k, n, n0, alpha, kleft, kright);
   	}
}