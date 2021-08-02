use super::*;
use eletric_field::EletricField;
use cores::Core;
use fp::list;
use fp::Matrix;
use itertools::izip;

//Nome de funções e variaveis precisa enfatizar o algoritmo da direção implicita

impl<T: Core<3>> Slab<T,3,2> {
	
	pub fn run(&self) -> EletricField {
		let &[zdepht, ydepht, xdepht] = self.core.get_shape();
		let &[_, ydelta, xdelta] = self.core.get_deltas();
	
		let e_input = self.get_input_beam();
	
		let es = (1usize..zdepht).fold(
			vec![e_input], 
			|result, z| {
				
				let last_es = fp::last(result.iter()).unwrap();
	
				let ds = (1..xdepht-1).map(
					|x| {
						self.get_d_vec(last_es, ydelta, z-1, [0,x], [ydepht-1,x]).into_iter().map(|d| d * self.dx2bydy2()).collect()
				}).collect();
				// criar função que transpõe
				let transposed_d_plane = matrix::new2_from_vec_vec(ds);

				let es_list = (0..ydepht).map(|y| {
			
					if y == 0 || y == ydepht - 1 {
						matrix::new(vec![*zero();xdepht])
					} else {
						self.get_es(
							self.equation_to_diagonal_matrix(self.get_s(xdelta,[z,y,1], [z,y,xdepht-2]), &get_line(last_es, [y,1], [y,xdepht-2])),
							//slice de cima -1
							get_line(&transposed_d_plane,[0,y-1],[xdepht-3,y-1])
						)
					}
		
				}).collect();
				let e_intermediate = matrix::new_from_vec(es_list);
				
				let hs = (1..ydepht-1).map(|y| {
					self.get_d_vec(&e_intermediate, xdelta, z-1, [y, 0], [y, xdepht-1]).into_iter().map(|d| d * self.dy2bydx2()).collect()
				}).collect();
				let h_plane = matrix::new2_from_vec_vec(hs);

				let es_list = (1..xdepht-1).map(|x|{
			
					self.get_es(
						self.equation_to_diagonal_matrix(self.get_s(ydelta, [z,1,x], [z,ydepht-2,x]), &get_line(&e_intermediate, [0, x], [ydepht-3, x])),
						//slice de cima -1
						get_line(&h_plane, [0, x-1], [ydepht-3, x-1])
					)
		
				}).collect();
				let e_transposed = matrix::new_from_vec(es_list);

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

	fn get_d_vec(&self, e_plane: &Matrix<Phasor>, delta: f64, z: usize, [y0,x0]: [usize;2], [y,x]:[usize;2]) -> Vec<Phasor> {
		let mut q = vec![];
		let mut es = vec![];

		for[_y, _x] in get_slice([y0,x0], [y,x]) {
			q.push(self.get_q(delta, [z, _y, _x]));
			es.push(e_plane.get(&[_y, _x]).clone())
		}
		
		get_ds(&es,q)
	} 

	fn dx2bydy2(&self)  -> Phasor {
		let &[_, ydelta, xdelta] = self.core.get_deltas();
		Complex::new(xdelta.powf(2.0) / ydelta.powf(2.0), 0.0)
	}

	fn dy2bydx2(&self) -> Phasor {
		let &[_, ydelta, xdelta] = self.core.get_deltas();
		Complex::new(ydelta.powf(2.0) / xdelta.powf(2.0), 0.0)
	}

	fn get_e_plane(&self, e_transposed: Matrix<Phasor>) -> Matrix<Phasor> {
		let &[_, ydepht, xdepht] = self.core.get_shape();

		let es_list = (0..ydepht).map(
			
			|y| self.insert_boundary_values(get_line(&e_transposed, [0, y], [xdepht-3, y]))
		
		).collect();
		
		matrix::new_from_vec(es_list)
	}
	
	fn get_s(&self, delta: f64, from: [usize;3], to: [usize;3]) -> Vec<Phasor> {
		let k = self.beam.k;
		let alpha = self.beam.alpha;
	
		let &[zdelta, _, _] = self.core.get_deltas();
		get_slice(from, to).map(
			|position| {
				self.s(position, zdelta, delta, k, alpha)
			}
		).collect()
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

fn get_line(m: &Matrix<Phasor>, from: [usize;2], to: [usize;2]) -> Vec<Phasor> {
	get_slice(from, to).map(|position| m.get(&position).clone()).collect()
}

fn get_slice<const N: usize>(from: [usize;N], to: [usize;N]) -> impl Iterator<Item = [usize;N]>  {
	let sub_matrix_shape = izip!(to, from).map(
		|(j, i)| {
			let d = j - i + 1;
			if d <= 0 {
				panic!("final indexes must be bigger or equal than initial indexes")
			}

			d
		}
	).collect();

	matrix::cartesian_product_of_shape(sub_matrix_shape).map(
		move |cursor| {
			let mut position = [0usize;N];
			for i in 0..N {
				position[i] = cursor[i] + from[i];
			}
			position
		}
	)
}