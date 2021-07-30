use super::*;
use eletric_field::EletricField;
use cores::Core;
use fp::list;
use fp::Matrix;

//Nome de funções e variaveis precisa enfatizar o algoritmo da direção implicita

impl<T: Core<3>> Slab<T,3,2> {
	
	pub fn run(&self) -> EletricField {
		let [zdepht, _, _] = self.core.get_shape().clone();
	
		let e_input = self.get_input_beam();
	
		let es = (1usize..zdepht).fold(
			vec![e_input], 
			|result, z| {
				
				let last_es = fp::last(result.iter()).unwrap();
	
				let e_intermediate = self.get_e_intermediate(
					last_es,
					self.get_transposed_d_plane(last_es, z-1),
					z-1
				);
				
				let e_transposed = self.get_e_transposed(
					&e_intermediate,
					self.get_h_plane(&e_intermediate, z-1),
					z-1
				);

				let e = self.get_e_plane(e_transposed);

				list::append(result, e)
			}
		);
	
		eletric_field::new (matrix::new_from_vec(es), self.core.get_deltas().to_vec())
	}
	
	fn get_input_beam(&self) -> Matrix<Phasor> {
		let shape  = self.core.get_shape();
		let deltas = self.core.get_deltas();

		beam::input(&[shape[1], shape[2]], &[deltas[1], deltas[2]], &self.beam.center, self.beam.amplitude, self.beam.width)
	}

	fn get_transposed_d_plane(&self, last_es: &Matrix<Phasor>, z: usize) -> Matrix<Phasor> {
		let &[_, _, xdepht] = self.core.get_shape();

		let d_list = (1..xdepht-1).map(|x| {
			get_ds(
				&get_col(last_es, x),
				self.get_qy(z, x)
			).into_iter().map(|d| d * self.dx2bydy2()).collect()
		}).collect();
		
		matrix::new2_from_vec_vec(d_list)
	}

	fn dx2bydy2(&self)  -> Phasor {
		let &[_, ydelta, xdelta] = self.core.get_deltas();
		Complex::new(xdelta.powf(2.0) / ydelta.powf(2.0), 0.0)
	}

	fn get_e_intermediate(&self, last_es: &Matrix<Phasor>, transposed_d_plane: Matrix<Phasor>, z: usize) -> Matrix<Phasor> {
		let &[_, ydepht, _] = self.core.get_shape();

		let es_list = (1..ydepht-1).map(|y| {
			
			self.get_es(
				self.equation_to_diagonal_matrix(self.get_sx(z, y), &get_row(last_es, y)),
				get_col(&transposed_d_plane,y-1)
			)

		}).collect();
		
		matrix::new_from_vec(es_list)
	}

	fn get_h_plane(&self, e_intermediate: &Matrix<Phasor>, z: usize) -> Matrix<Phasor> {
		let &[_, ydepht, _] = self.core.get_shape();

		let h_list = (1..ydepht-1).map(|y|{
			get_ds(
				&get_row(&e_intermediate, y-1),
				self.get_qx(z, y)
			).into_iter().map(|d| d * self.dy2bydx2()).collect()
		}).collect();
		
		matrix::new2_from_vec_vec(h_list)
	}

	fn dy2bydx2(&self) -> Phasor {
		let &[_, ydelta, xdelta] = self.core.get_deltas();
		Complex::new(ydelta.powf(2.0) / xdelta.powf(2.0), 0.0)
	}

	fn get_e_transposed(&self, e_intermediate: &Matrix<Phasor>, h_plane: Matrix<Phasor>, z: usize) -> Matrix<Phasor> {
		let &[_, _, xdepht] = self.core.get_shape();

		let es_list = (1..xdepht-1).map(|x|{
			
			self.get_es(
				self.equation_to_diagonal_matrix(self.get_sy(z, x), &get_col(&e_intermediate, x)),
				get_col(&h_plane, x-1)
			)

		}).collect();
		
		matrix::new_from_vec(es_list)
	}

	fn get_e_plane(&self, e_transposed: Matrix<Phasor>) -> Matrix<Phasor> {
		let &[_, ydepht, _] = self.core.get_shape();

		let es_list = (0..ydepht).map(
			
			|y| self.insert_boundary_values(get_col(&e_transposed, y))
		
		).collect();
		
		matrix::new_from_vec(es_list)
	}
	
	fn get_sx(&self, z: usize, y: usize) -> Vec<Phasor> {
		let k = self.beam.k;
		let alpha = self.beam.alpha;
	
		let &[zdelta, _, xdelta] = self.core.get_deltas();
		let &[_, _, xdepht] = self.core.get_shape();
	
		(1..xdepht-1).map(|x| {
			self.s([z, y, x], zdelta, xdelta, k, alpha)
		}).collect()
	}
	
	fn get_qx(&self, z: usize, y: usize) -> Vec<Phasor> {
		let k = self.beam.k;
		let alpha = self.beam.alpha;
	
		let &[zdelta, _, xdelta] = self.core.get_deltas();
		let &[_, _, xdepht] = self.core.get_shape();
	
		(1..xdepht-1).map(|x| {
			self.q([z, y, x], zdelta, xdelta, k, alpha)
		}).collect()
	}
	
	fn get_sy(&self, z: usize, x: usize) -> Vec<Phasor> {
		let k = self.beam.k;
		let alpha = self.beam.alpha;
	
		let &[zdelta, ydelta, _] = self.core.get_deltas();
		let &[_, ydepht, _] = self.core.get_shape();
	
		(1..ydepht-1).map(|y| {
			self.s([z, y, x], zdelta, ydelta, k, alpha)
		}).collect()
	}
	
	fn get_qy(&self, z: usize, x: usize) -> Vec<Phasor> {
		let k = self.beam.k;
		let alpha = self.beam.alpha;
	
		let &[zdelta, ydelta, _] = self.core.get_deltas();
		let &[_, ydepht, _] = self.core.get_shape();
	
		(1..ydepht-1).map(|y| {
			self.q([z, y, x], zdelta, ydelta, k, alpha)
		}).collect()
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

// #Todo Otimizar submatrix pra usa la
fn get_col(m: &Matrix<Phasor>, x: usize) -> Vec<Phasor> {
	// temporario vai usar como garantia que m tem depht 2
	let y_depht = m.shape()[0];

	(0..y_depht).map(|y| m.get(&[y, x]).clone()).collect()
}

// #Todo Otimizar submatrix pra usa la
fn get_row(m: &Matrix<Phasor>, y: usize) -> Vec<Phasor> {
	// temporario vai usar como garantia que m tem depht 2
	let x_depht = m.shape()[1];

	(0..x_depht).map(|x| m.get(&[y, x]).clone()).collect()
}