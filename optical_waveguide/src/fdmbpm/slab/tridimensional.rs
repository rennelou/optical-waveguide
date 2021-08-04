use super::*;
use eletric_field::EletricField;
use cores::Core;
use fp::list;

//Nome de funções e variaveis precisa enfatizar o algoritmo da direção implicita

impl<T: Core<3>> Slab<T,3,2> {
	
	pub fn run(&self) -> EletricField {
		let &[zdepht, ydepht, xdepht] = self.core.get_shape();
		let &[_, ydelta, xdelta] = self.core.get_deltas();
	
		let e_input = self.beam.input(&[ydepht, xdepht], &[ydelta, xdelta]);
	
		let es = (1usize..zdepht).fold(
			vec![e_input], 
			|result, z| {
				
				let last_es = fp::last(result.iter()).unwrap();
	
				let d_plane: Vec<Vec<_>> = (1..xdepht-1).map(
					|x| {
						let mut q = vec![];
						let mut es = vec![];

						for y in 0..ydepht {
							q.push(self.get_q(ydelta, [z-1, y, x]));
							es.push(last_es.get(&[y, x]).clone())
						}
		
						get_const_terms(&es,q).into_iter().map(|d| d * self.dx2bydy2()).collect()
				}).collect();

				let mut e_intermediate = vec![];
				let mut h_plane: Vec<Vec<_>>  = vec![];
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
							d.push(d_plane[y-1][x-1]);
						}
					}

					let e = self.get_es(self.equation_to_diagonal_matrix(s, &es), d);
					let h = get_const_terms(&e ,q).into_iter().map(|d| d * self.dy2bydx2()).collect();

					e_intermediate.push(e);
					h_plane.push(h);
				}

				let e_transposed: Vec<Vec<_>> = (1..xdepht-1).map(|x|{
					let mut s = vec![];
					let mut es = vec![];
					let mut d = vec![];

					for y in 1..ydepht-1 {
						s.push(self.get_s(ydelta, [z-1, y, x]));
						es.push(e_intermediate[y-1][x]);
						d.push(h_plane[y-1][x-1]);
					}

					self.get_es(self.equation_to_diagonal_matrix(s, &es), d)

				}).collect();
				let es_middle = matrix::transposed_vec2_to_matrix2(e_transposed);

				let es = (0..ydepht).map(
					|y| {
						let mut e_middle_line = vec![];
						for x in 0..xdepht-2 {
							e_middle_line.push(es_middle.get(&[y,x]).clone());
						}
						self.insert_boundary_values(e_middle_line)
					}
				).collect();
				let e = matrix::vec2_to_matrix2(es);

				list::append(result, e)
			}
		);
	
		eletric_field::new (matrix::merge(es), self.core.get_deltas().to_vec())
	}
	
	fn dx2bydy2(&self)  -> Phasor {
		let &[_, ydelta, xdelta] = self.core.get_deltas();
		Complex::new(xdelta.powf(2.0) / ydelta.powf(2.0), 0.0)
	}

	fn dy2bydx2(&self) -> Phasor {
		let &[_, ydelta, xdelta] = self.core.get_deltas();
		Complex::new(ydelta.powf(2.0) / xdelta.powf(2.0), 0.0)
	}

	fn get_s(&self, delta: f64, position: [usize;3]) -> Phasor {
		let k = self.beam.k;
		let alpha = self.beam.alpha;
	
		let &[zdelta, _, _] = self.core.get_deltas();
		
		self.s(position, zdelta, delta, k, alpha)
	}
	
	fn get_q(&self, delta: f64, position: [usize;3]) -> Phasor {
		let k = self.beam.k;
		let alpha = self.beam.alpha;
	
		let &[zdelta, _, _] = self.core.get_deltas();
		
		self.q(position, zdelta, delta, k, alpha)
	}
}

impl<T: Core<3>> SlabParamtersFormulas<T,3> for Slab<T,3,2> {
	
	fn guiding_space(&self, position: [usize;3], delta: f64, k: f64) -> f64 {
		let n0 = self.core.get_n0();
	
		0.5*k.powf(2.0)*delta.powf(2.0)*(self.core.get_half_n(&position, n0).powf(2.0)-n0.powf(2.0))
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