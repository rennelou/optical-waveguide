use super::*;
use eletric_field::EletricField;
use cores::Core;
use crate::fp::matrix;
use crate::fp::list;

impl<T: Core<2>> Slab<T,2,1> {
	
	pub fn run(&self) -> EletricField {
		let &[zdepht, _] = self.core.get_shape();
	
		let e_input = self.get_input_beam();
	
		let es = (1usize..zdepht).fold( 
			vec![e_input],
			|acc, z| {
				
				let last_es= fp::last(acc.iter()).unwrap().raw();
				
				let e = self.get_es(
					self.equation_to_diagonal_matrix(self.get_s(z-1), last_es),
					get_ds(last_es, self.get_q(z-1))
				);
	
				list::append(acc, e)
			}
		);
	
		eletric_field::new(matrix::new_from_vec(es), self.core.get_deltas().to_vec())
	}

	fn get_input_beam(&self) -> Matrix<Phasor> {
		beam::input(&[self.core.get_shape()[1]], &[self.core.get_deltas()[1]], &self.beam.center, self.beam.amplitude, self.beam.width)
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
	
		(0..x_depht).map(|x| {
			self.q([z, x], zdelta, xdelta, k, alpha)
		}).collect()
	}
}

impl<T: Core<2>> SlabParamtersFormulas<T,2> for Slab<T,2,1> {
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
}