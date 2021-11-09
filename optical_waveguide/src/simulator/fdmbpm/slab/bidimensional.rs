use super::*;
use fp::matrix;
use eletric_field::EletricField;
use grid::AlTypeGrid;
use cores::AlTypeCore;

impl Slab<2,1> {
	
	pub fn run(self) -> EletricField {
		let &[zdepht, _] = self.grid.get_shape();
	
		let e_input = self.beam.input(&[self.grid.get_shape()[1]], &[self.grid.get_deltas()[1]]);
	
		let es = (1usize..zdepht).fold( 
			vec![e_input],
			|acc, z| {
				
				let last_es= fp::last(acc.iter()).unwrap().raw();
				
				let e = self.get_es(
					self.equation_to_diagonal_matrix(self.get_s(z-1), last_es),
					get_const_terms(last_es, self.get_q(z-1))
				);

				list::append(acc, matrix::vec_to_matrix(e))
			}
		);
	
		eletric_field::new(
			matrix::merge(es), 
			self.grid.get_deltas().to_vec(), 
			AlTypeGrid::Bidimensional(self.grid),
			AlTypeCore::Bidimensional(self.core)
		)
	}

	fn get_s(&self, z: usize) -> Vec<Phasor> {
		let k = self.beam.k;
		let alpha = self.beam.alpha;
	
		let [_, x_depht] = self.grid.get_shape().clone();
		let &[zdelta, xdelta] = self.grid.get_deltas();
	
		(1..x_depht-1).map(|x| {
			self.s([z, x], zdelta, xdelta, k, alpha)
		}).collect()
	}
	
	fn get_q(&self, z: usize) -> Vec<Phasor> {
		let k = self.beam.k;
		let alpha = self.beam.alpha;
	
		let [_, x_depht] = self.grid.get_shape().clone();
		let &[zdelta, xdelta] = self.grid.get_deltas();
	
		(0..x_depht).map(|x| {
			self.q([z, x], zdelta, xdelta, k, alpha)
		}).collect()
	}
}

impl SlabParamtersFormulas<2> for Slab<2,1> {
	fn guiding_space(&self, position: [usize;2], delta: f64, k: f64) -> f64 {
		let &n0 = &self.core.get_n0();
	
		k.powf(2.0)*delta.powf(2.0)*(&self.core.get_half_n(&self.grid, &position, n0).powf(2.0)-n0.powf(2.0))
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