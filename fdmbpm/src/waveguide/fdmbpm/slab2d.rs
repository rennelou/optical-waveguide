use super::*;
use eletric_field::EletricField;
use cores::Core;
use crate::fp::matrix;
use crate::fp::list;

impl<T: Core<2>> Slab<T,2,1> {
	pub fn run(&self) -> EletricField {
		let shape = self.core.get_shape();
		let zsteps = shape[0];
	
		let e_input = self.get_input_beam();
	
		let es = (1usize..zsteps).fold( 
			vec![e_input],
			|acc, z| {
				
				let last_es= fp::last(acc.iter()).unwrap().raw();
				let last_q = self.get_q(z-1);
				let last_s = self.get_s(z-1);
				
				let matrix = equation_to_diagonal_matrix(last_s, last_es, self.boundary_codition);
				let e = get_es(matrix, get_ds(last_es, last_q), self.boundary_codition);
	
				list::append(acc, e)
			}
		);
	
		eletric_field::new(matrix::new_from_vec(es), self.core.get_deltas().to_vec())
	}

	fn get_input_beam(&self) -> Matrix<Phasor> {
		waves::input(&[self.core.get_shape()[1]], &[self.core.get_deltas()[1]], &self.beam.center, self.beam.amplitude, self.beam.width)
	}

	fn get_s(&self, z: usize) -> Vec<Phasor> {
		let k = self.beam.k;
		let alpha = self.beam.alpha;
	
		let [_, x_depht] = self.core.get_shape().clone();
		let &[zdelta, xdelta] = self.core.get_deltas();
	
		(1..x_depht-1).map(|x| {
			self.s([z, x], zdelta, xdelta, k, alpha)
		}).collect()
	}
	
	fn get_q(&self, z: usize) -> Vec<Phasor> {
		let k = self.beam.k;
		let alpha = self.beam.alpha;
	
		let [_, x_depht] = self.core.get_shape().clone();
		let &[zdelta, xdelta] = self.core.get_deltas();
	
		(1..x_depht-1).map(|x| {
			self.q([z, x], zdelta, xdelta, k, alpha)
		}).collect()
	}

	fn guiding_space(&self, position: [usize;2], delta: f64, k: f64) -> f64 {
		let &n0 = &self.core.get_n0();
	
		k.powf(2.0)*delta.powf(2.0)*(&self.core.get_half_n(&position, n0).powf(2.0)-n0.powf(2.0))
	}
	
	fn free_space(&self, zdelta: f64, delta: f64, k: f64) -> f64 {
		let n0 = self.core.get_n0();
		
		4.0*k*n0*delta.powf(2.0)/zdelta
	}
	
	fn loss(&self, delta: f64, k: f64, alpha: f64) -> f64 {
		let n0 = &self.core.get_n0();
	
		2.0*k*n0*delta.powf(2.0)*alpha
	}
	
	// Todo essas funções serão compartilhadas entre slab2d e slab3d
	fn s(&self, position: [usize;2], zdelta: f64, delta: f64, k: f64, alpha: f64) -> Phasor {
		let (guiding_space, free_space, loss) = self.slab_formulas(position, zdelta, delta, k, alpha);
		Complex::new(2.0 - guiding_space, free_space + loss)
	}
	
	fn q(&self, position: [usize;2], zdelta: f64, delta: f64, k: f64, alpha: f64) -> Phasor {
		let (guiding_space, free_space, loss) = self.slab_formulas(position, zdelta, delta, k, alpha);
		Complex::new(-2.0 + guiding_space, free_space - loss)
	}
	
	fn slab_formulas(&self, position: [usize;2], zdelta: f64, delta: f64, k: f64, alpha: f64) -> (f64, f64, f64) {
		let guiding_space = self.guiding_space(position, delta, k);
		let free_space = self.free_space(zdelta, delta, k);
		let loss = self.loss(delta, k, alpha);
	
		(guiding_space, free_space, loss)
	}
}