use super::*;
use eletric_field::EletricField;
use cores::Core;
use fp::list;
use fp::Matrix;

impl<T: Core<3>> Slab<T,3,2> {
	
	pub fn run(&self) -> EletricField {
		let [zdepht, ydepht, xdepht] = self.core.get_shape().clone();
	
		let e_input = self.get_input_beam();
	
		let es = (1usize..zdepht).fold(
			vec![e_input], 
			|result, z| {
				
				let last_es = fp::last(result.iter()).unwrap();
	
				let d_list = (1..xdepht-1).map(|x| {
					let last_es_col= get_col(last_es, x);
					
					let last_qy = self.get_qy(z-1, x,);
					
					get_ds(&last_es_col, last_qy).into_iter().map(|e| e * self.dx2bydy2()).collect()
				}).collect();
				let transposed_d_plane = matrix::new2_from_vec_vec(d_list);
	
				let es_list = (1..ydepht-1).map(|y| {
					let last_es_row= get_row(last_es, y);
	
					let sx_list = self.get_sx(z-1, y);
					let d_list = get_col(&transposed_d_plane,y-1);
	
					let matrix = equation_to_diagonal_matrix(sx_list, &last_es_row, self.boundary_codition);
					get_es(matrix, d_list, self.boundary_codition)
	
				}).collect();
				let es_intermediate = matrix::new_from_vec(es_list);
				
	//----------------------- segunda parte -----------------------------------------------
	
				let h_list = (1..ydepht-1).map(|y|{
					let es_intermediate_row = get_row(&es_intermediate, y-1);
					let last_qx = self.get_qx(z-1, y);
	
					get_ds(&es_intermediate_row, last_qx).into_iter().map(|e| e * self.dy2bydx2()).collect()
				}).collect();
				let h_plane = matrix::new2_from_vec_vec(h_list);
				
				let es_list = (1..xdepht-1).map(|x|{
					let es_intermediate_col= get_col(&es_intermediate, x);
	
					let sy_list = self.get_sy(z-1, x);
					let h_list = get_col(&h_plane, x-1);
	
					let matrix = equation_to_diagonal_matrix(sy_list, &es_intermediate_col, self.boundary_codition);
					get_es(matrix, h_list, self.boundary_codition)
	
				}).collect();
				let es_transposed = matrix::new_from_vec(es_list);
				
				let es_list = (0..ydepht).map(|y|{
					let es_to_insert_boundary_x = get_col(&es_transposed, y);
					insert_boundary_values(es_to_insert_boundary_x, self.boundary_codition)
				}).collect();
				let es = matrix::new_from_vec(es_list);
	
				list::append(result, es)
			}
		);
	
		eletric_field::new (matrix::new_from_vec(es), self.core.get_deltas().to_vec())
	}
	
	fn get_input_beam(&self) -> Matrix<Phasor> {
		let shape  = self.core.get_shape();
		let deltas = self.core.get_deltas();

		waves::input(&[shape[1], shape[2]], &[deltas[1], deltas[2]], &self.beam.center, self.beam.amplitude, self.beam.width)
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
	
	// Todo essas funções serão compartilhadas entre slab2d e slab3d
	fn s(&self, position: [usize;3], zdelta: f64, delta: f64, k: f64, alpha: f64) -> Phasor {
		let (guiding_space, free_space, loss) = self.slab_formulas(position, zdelta, delta, k, alpha);
		Complex::new(2.0 - guiding_space, free_space + loss)
	}
	
	fn q(&self, position: [usize;3], zdelta: f64, delta: f64, k: f64, alpha: f64) -> Phasor {
		let (guiding_space, free_space, loss) = self.slab_formulas(position, zdelta, delta, k, alpha);
		Complex::new(-2.0 + guiding_space, free_space - loss)
	}
	
	fn slab_formulas(&self, position: [usize;3], zdelta: f64, delta: f64, k: f64, alpha: f64) -> (f64, f64, f64) {
		let guiding_space = self.guiding_space(position, delta, k);
		let free_space = self.free_space(zdelta, delta, k);
		let loss = self.loss(delta, k, alpha);
	
		(guiding_space, free_space, loss)
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