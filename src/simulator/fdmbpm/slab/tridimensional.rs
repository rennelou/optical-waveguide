use super::*;
use functional_types::{Matrix, matrix};
use eletric_field::EletricField;
use functional_types::list;

impl WaveguideSimulation for Slab<3,2> {
	
	fn run(self) -> EletricField {
		let &[zdepht, ydepht, xdepht] = self.grid.get_shape();
		let &[_, ydelta, xdelta] = self.grid.get_deltas();
	
		let e_input = self.beam.input(&[ydepht, xdepht], &[ydelta, xdelta]);
	
		let es = (1usize..zdepht).fold(
			vec![e_input], 
			|result, z| {
				
				let last_es = functional_types::last(result.iter()).unwrap();
				
				let e = self.alternate_direction_implicit_method(last_es, z);

				list::append(result, e)
			}
		);
	
		eletric_field::new(
			matrix::merge(es), 
			self.grid.get_deltas().to_vec(),
			self.get_refractive_indexes()
		)
	}
}

impl Slab<3,2> {

	fn alternate_direction_implicit_method(&self, last_es: &Matrix<Phasor>, z: usize) -> Matrix<Phasor> {
		let &[_, ydepht, xdepht] = self.grid.get_shape();
		let &[_, ydelta, xdelta] = self.grid.get_deltas();

		let const_y_explicit_plane: Vec<Vec<_>> = (1..xdepht-1).map(
			|x| {
				let mut q = vec![];
				let mut es = vec![];

				for y in 0..ydepht {
					q.push(self.get_q(ydelta, [z-1, y, x]));
					es.push(last_es.get(&[y, x]).clone())
				}

				get_const_terms(&es,q).into_iter().map(|d| d * self.dx2bydy2()).collect()
		}).collect();

		let mut e_x_expicit = vec![];
		let mut const_x_explicit_plane: Vec<Vec<_>>  = vec![];
		for y in 1..ydepht-1 {
	
			let mut s = vec![];
			let mut q = vec![];
			let mut es = vec![];
			let mut d = vec![];

			for x in 0..xdepht {
				q.push(self.get_q(xdelta, [z-1, y, x]));
				
				if x > 0 && x < xdepht-1 {
					s.push(self.get_s(xdelta, [z-1, y, x]));
				
					es.push(last_es.get(&[y, x]).clone());
					d.push(const_y_explicit_plane[y-1][x-1]);
				}
			}

			let e = self.get_es(self.equation_to_diagonal_matrix(s, &es), d);
			let h = get_const_terms(&e ,q).into_iter().map(|d| d * self.dy2bydx2()).collect();

			e_x_expicit.push(e);
			const_x_explicit_plane.push(h);
		}

		let e_transposed_y_expicit: Vec<Vec<_>> = (1..xdepht-1).map(|x|{
			let mut s = vec![];
			let mut es = vec![];
			let mut d = vec![];

			for y in 1..ydepht-1 {
				s.push(self.get_s(ydelta, [z-1, y, x]));
				es.push(e_x_expicit[y-1][x]);
				d.push(const_x_explicit_plane[y-1][x-1]);
			}

			self.get_es(self.equation_to_diagonal_matrix(s, &es), d)

		}).collect();
		
		let es_middle = matrix::transposed_vec2_to_matrix2(e_transposed_y_expicit);

		let es = (0..ydepht).map(
			|y| {
				let mut e_middle_line = vec![];
				for x in 0..xdepht-2 {
					e_middle_line.push(es_middle.get(&[y,x]).clone());
				}
				self.insert_boundary_values(e_middle_line)
			}
		).collect();

		matrix::vec2_to_matrix2(es)
	}

	fn dx2bydy2(&self)  -> Phasor {
		let &[_, ydelta, xdelta] = self.grid.get_deltas();
		Complex::new(xdelta.powf(2.0) / ydelta.powf(2.0), 0.0)
	}

	fn dy2bydx2(&self) -> Phasor {
		let &[_, ydelta, xdelta] = self.grid.get_deltas();
		Complex::new(ydelta.powf(2.0) / xdelta.powf(2.0), 0.0)
	}

	fn get_s(&self, delta: f64, position: [usize;3]) -> Phasor {
		let k = self.beam.k;
		let alpha = self.beam.alpha;
	
		let &[zdelta, _, _] = self.grid.get_deltas();
		
		self.s(position, zdelta, delta, k, alpha)
	}
	
	fn get_q(&self, delta: f64, position: [usize;3]) -> Phasor {
		let k = self.beam.k;
		let alpha = self.beam.alpha;
	
		let &[zdelta, _, _] = self.grid.get_deltas();
		
		self.q(position, zdelta, delta, k, alpha)
	}
}

impl SlabParamtersFormulas<3> for Slab<3,2> {
	
	fn guiding_space(&self, position: [usize;3], delta: f64, k: f64) -> f64 {
		let n0 = self.core.get_n0();
	
		0.5*k.powf(2.0)*delta.powf(2.0)*(self.core.get_half_n(&self.grid, &position, n0).powf(2.0)-n0.powf(2.0))
	}
	
	fn free_space(&self, zdelta: f64, delta: f64, k: f64) -> f64 {
		let n0 = self.core.get_n0();
		
		4.0*k*n0*delta.powf(2.0)/zdelta
	}
	
	fn loss(&self, delta: f64, k: f64, alpha: f64) -> f64 {
		let n0 = self.core.get_n0();
	
		k*n0*delta.powf(2.0)*alpha
	}
}