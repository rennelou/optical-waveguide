use super::*;
use eletric_field::EletricField;
use cores::Core;
use fp::list;
use fp::Matrix;

impl<T: Core<3>> Slab<T,3,2> {
	
	pub fn run(&self) -> EletricField {
		let [zdepht, _, _] = self.core.get_shape().clone();
	
		let e_input = self.get_input_beam();
	
		let es = (1usize..zdepht).fold(
			vec![e_input], 
			|result, z| {
				
				let last_es = fp::last(result.iter()).unwrap();
	
				let transposed_d_plane = self.get_transposed_d_plane(last_es, z-1);
				let e_intermediate = self.get_e_intermediate(last_es, transposed_d_plane, z-1);
				
				let h_plane = self.get_h_plane(&e_intermediate, z-1);
				let e_transposed = self.get_e_transposed(&e_intermediate, h_plane, z-1);

				let e = self.get_e_plane(e_transposed);

				list::append(result, e)
			}
		);
	
		eletric_field::new (matrix::new_from_vec(es), self.core.get_deltas().to_vec())
	}
	
	fn get_input_beam(&self) -> Matrix<Phasor> {
		let shape  = self.core.get_shape();
		let deltas = self.core.get_deltas();

		waves::input(&[shape[1], shape[2]], &[deltas[1], deltas[2]], &self.beam.center, self.beam.amplitude, self.beam.width)
	}

	fn get_transposed_d_plane(&self, last_es: &Matrix<Phasor>, z: usize) -> Matrix<Phasor> {
		let &[_, _, xdepht] = self.core.get_shape();

		let d_list = (1..xdepht-1).map(|x| {
			let last_es_col= get_col(last_es, x);
			
			let last_qy = self.get_qy(z, x);
			
			get_ds(&last_es_col, last_qy).into_iter().map(|e| e * self.dx2bydy2()).collect()
		}).collect();
		
		matrix::new2_from_vec_vec(d_list)
	}

	fn get_e_intermediate(&self, last_es: &Matrix<Phasor>, transposed_d_plane: Matrix<Phasor>, z: usize) -> Matrix<Phasor> {
		let &[_, ydepht, _] = self.core.get_shape();

		let es_list = (1..ydepht-1).map(|y| {
			let last_es_row= get_row(last_es, y);

			let sx_list = self.get_sx(z, y);
			let d_list = get_col(&transposed_d_plane,y-1);

			let matrix = equation_to_diagonal_matrix(sx_list, &last_es_row, self.boundary_codition);
			get_es(matrix, d_list, self.boundary_codition)

		}).collect();
		
		matrix::new_from_vec(es_list)
	}

	fn get_h_plane(&self, e_intermediate: &Matrix<Phasor>, z: usize) -> Matrix<Phasor> {
		let &[_, ydepht, _] = self.core.get_shape();

		let h_list = (1..ydepht-1).map(|y|{
			let es_intermediate_row = get_row(&e_intermediate, y-1);
			let last_qx = self.get_qx(z, y);

			get_ds(&es_intermediate_row, last_qx).into_iter().map(|e| e * self.dy2bydx2()).collect()
		}).collect();
		
		matrix::new2_from_vec_vec(h_list)
	}

	fn get_e_transposed(&self, e_intermediate: &Matrix<Phasor>, h_plane: Matrix<Phasor>, z: usize) -> Matrix<Phasor> {
		let &[_, _, xdepht] = self.core.get_shape();

		let es_list = (1..xdepht-1).map(|x|{
			let es_intermediate_col= get_col(&e_intermediate, x);

			let sy_list = self.get_sy(z, x);
			let h_list = get_col(&h_plane, x-1);

			let matrix = equation_to_diagonal_matrix(sy_list, &es_intermediate_col, self.boundary_codition);
			get_es(matrix, h_list, self.boundary_codition)

		}).collect();
		
		matrix::new_from_vec(es_list)
	}

	fn get_e_plane(&self, e_transposed: Matrix<Phasor>) -> Matrix<Phasor> {
		let &[_, ydepht, _] = self.core.get_shape();

		let es_list = (0..ydepht).map(|y|{
			let es_to_insert_boundary_x = get_col(&e_transposed, y);
			insert_boundary_values(es_to_insert_boundary_x, self.boundary_codition)
		}).collect();
		
		matrix::new_from_vec(es_list)
	}

	fn dy2bydx2(&self) -> Phasor {
		let &[_, ydelta, xdelta] = self.core.get_deltas();
		Complex::new(ydelta.powf(2.0) / xdelta.powf(2.0), 0.0)
	}

	fn dx2bydy2(&self)  -> Phasor {
		let &[_, ydelta, xdelta] = self.core.get_deltas();
		Complex::new(xdelta.powf(2.0) / ydelta.powf(2.0), 0.0)
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