use crate::fp::matrix::Idx;
use crate::waves;
use crate::waves::Gaussian;

use super::*;
use cores::Core;
use Phasor;
use fp::list;
use fp::Matrix;

pub fn run(core: &impl Core<3>, beam: Gaussian<2>, boundary_codition: fn(s: Side, es: &Vec<Phasor>)-> Phasor) -> EletricField<3> {
	let [zdepht, ydepht, xdepht] = core.get_shape().clone();
	let shape  = core.get_shape();
	let deltas = core.get_deltas();

	let e_input = waves::input(&[shape[1], shape[2]], &[deltas[1], deltas[2]], &beam.center, beam.amplitude, beam.width);

	let &[_, ydelta, xdelta] = core.get_deltas();
	let dy2bydx2 = Complex::new(ydelta.powf(2.0) / xdelta.powf(2.0), 0.0);
	let dx2bydy2 = Complex::new(xdelta.powf(2.0) / ydelta.powf(2.0), 0.0);

	let es = (1usize..zdepht).fold(
		vec![e_input], 
		|result, z| {
			
			let last_es = fp::last(result.iter()).unwrap();

			let d_list = (1..xdepht-1).map(|x| {
				let last_es_col= get_col(last_es, x);
				
				let last_qy = get_qy(core, z-1, Idx::Free, Idx::Value(x), &beam);
				
				get_ds(&last_es_col, last_qy).into_iter().map(|e| e * dx2bydy2).collect()
			}).collect();
			let transposed_d_plane = matrix::new2_from_vec_vec(d_list);

			let es_list = (1..ydepht-1).map(|y| {
				let last_es_row= get_row(last_es, y);

				let sx_list = get_sx(core, z-1, Idx::Value(y), Idx::Free, &beam);
				let d_list = get_col(&transposed_d_plane,y-1);

				get_es(sx_list, d_list, &last_es_row, boundary_codition)
			}).collect();
			let es_intermediate = matrix::new_from_vec(es_list);
			
//----------------------- segunda parte -----------------------------------------------

			let h_list = (1..ydepht-1).map(|y|{
				let es_intermediate_row = get_row(&es_intermediate, y-1);
				let last_qx = get_qx(core, z-1, Idx::Value(y), Idx::Free, &beam);

				get_ds(&es_intermediate_row, last_qx).into_iter().map(|e| e * dy2bydx2).collect()
			}).collect();
			let h_plane = matrix::new2_from_vec_vec(h_list);
			
			let es_list = (1..xdepht-1).map(|x|{
				let es_intermediate_col= get_col(&es_intermediate, x);

				let sy_list = get_sy(core, z-1, Idx::Free, Idx::Value(x), &beam);
				let h_list = get_col(&h_plane, x-1);

				get_es(sy_list, h_list, &es_intermediate_col, boundary_codition)
			}).collect();
			let es_transposed = matrix::new_from_vec(es_list);
			
			let es_list = (0..ydepht).map(|y|{
				let es_to_insert_boundary_x = get_col(&es_transposed, y);
				insert_boundary_values(es_to_insert_boundary_x, boundary_codition)
			}).collect();
			let es = matrix::new2_from_vec_vec(es_list);

			list::append(result, es)
		}
	);

	let values = matrix::new_from_vec(es);
	let &grid_steps = core.get_deltas();
	return EletricField { values, grid_steps };
}

fn get_col(m: &Matrix<Phasor,2>, x: usize) -> Vec<Phasor> {
	let &[y_depht, _] = m.shape();

	(0..y_depht).map(|y| m.get(&[y, x]).clone()).collect()
}

fn get_row(m: &Matrix<Phasor,2>, y: usize) -> Vec<Phasor> {
	let &[_, x_depht] = m.shape();

	(0..x_depht).map(|x| m.get(&[y, x]).clone()).collect()
}

fn get_sx(core: &impl Core<3>, z: usize, y_idx: Idx, x_idx: Idx, beam: &Gaussian<2>) -> Vec<Phasor> {
	let &[_, y_depht, x_depht] = core.get_shape();
	let &[zdelta, _, xdelta] = core.get_deltas();
	let k = beam.k;
	let alpha = beam.alpha;

	match (y_idx, x_idx) {
		(Idx::Value(_), Idx::Value(_)) => {
			panic!("get_s result needs have one dimension")
		},
		(Idx::Free, Idx::Free) => {
			panic!("get_s result needs have one dimension")
		},
		(Idx::Free, Idx::Value(x)) => {
			(0..y_depht).map(|y| {
				let guiding_space = guiding_space(core, z, y, x, xdelta, k);
				let free_space = free_space(core, zdelta, xdelta, k);
				let loss = loss(core, xdelta, k, alpha);
		
				Complex::new(2.0 - guiding_space, free_space + loss)
			}).collect()
		},
		(Idx::Value(y), Idx::Free) => {
			(0..x_depht).map(|x| {
				let guiding_space = guiding_space(core, z, y, x, xdelta, k);
				let free_space = free_space(core, zdelta, xdelta, k);
				let loss = loss(core, xdelta, k, alpha);
		
				Complex::new(2.0 - guiding_space, free_space + loss)
			}).collect()
		}
	}
}

fn get_qx(core: &impl Core<3>, z: usize, y_idx: Idx, x_idx: Idx, beam: &Gaussian<2>) -> Vec<Phasor> {
	let [_, y_depht, x_depht] = core.get_shape().clone();
	let &[zdelta, _, xdelta] = core.get_deltas();
	let k = beam.k;
	let alpha = beam.alpha;

	match (y_idx, x_idx) {
		(Idx::Value(_), Idx::Value(_)) => {
			panic!("get_q result needs have one dimension")
		},
		(Idx::Free, Idx::Free) => {
			panic!("get_q result needs have one dimension")
		},
		(Idx::Free, Idx::Value(x)) => {
			(0..y_depht).map(|y| {
				let guiding_space = guiding_space(core, z, y, x, xdelta, k);
				let free_space = free_space(core, zdelta, xdelta, k);
				let loss = loss(core, xdelta, k, alpha);
		
				Complex::new(-2.0 + guiding_space, free_space - loss)
			}).collect()
		},
		(Idx::Value(y), Idx::Free) => {
			(0..x_depht).map(|x| {
				let guiding_space = guiding_space(core, z, y, x, xdelta, k);
				let free_space = free_space(core, zdelta, xdelta, k);
				let loss = loss(core, xdelta, k, alpha);
		
				Complex::new(-2.0 + guiding_space, free_space - loss)
			}).collect()
		}
	}
}

fn get_sy(core: &impl Core<3>, z: usize, y_idx: Idx, x_idx: Idx, beam: &Gaussian<2>) -> Vec<Phasor> {
	let &[_, y_depht, x_depht] = core.get_shape();
	let &[zdelta, ydelta, _] = core.get_deltas();
	let k = beam.k;
	let alpha = beam.alpha;

	match (y_idx, x_idx) {
		(Idx::Value(_), Idx::Value(_)) => {
			panic!("get_s result needs have one dimension")
		},
		(Idx::Free, Idx::Free) => {
			panic!("get_s result needs have one dimension")
		},
		(Idx::Free, Idx::Value(x)) => {
			(0..y_depht).map(|y| {
				let guiding_space = guiding_space(core, z, y, x, ydelta, k);
				let free_space = free_space(core, zdelta, ydelta, k);
				let loss = loss(core, ydelta, k, alpha);
		
				Complex::new(2.0 - guiding_space, free_space + loss)
			}).collect()
		},
		(Idx::Value(y), Idx::Free) => {
			(0..x_depht).map(|x| {
				let guiding_space = guiding_space(core, z, y, x, ydelta, k);
				let free_space = free_space(core, zdelta, ydelta, k);
				let loss = loss(core, ydelta, k, alpha);
		
				Complex::new(2.0 - guiding_space, free_space + loss)
			}).collect()
		}
	}
}

fn get_qy(core: &impl Core<3>, z: usize, y_idx: Idx, x_idx: Idx, beam: &Gaussian<2>) -> Vec<Phasor> {
	let [_, y_depht, x_depht] = core.get_shape().clone();
	let &[zdelta, ydelta, _] = core.get_deltas();
	let k = beam.k;
	let alpha = beam.alpha;

	match (y_idx, x_idx) {
		(Idx::Value(_), Idx::Value(_)) => {
			panic!("get_q result needs have one dimension")
		},
		(Idx::Free, Idx::Free) => {
			panic!("get_q result needs have one dimension")
		},
		(Idx::Free, Idx::Value(x)) => {
			(0..y_depht).map(|y| {
				let guiding_space = guiding_space(core, z, y, x, ydelta, k);
				let free_space = free_space(core, zdelta, ydelta, k);
				let loss = loss(core, ydelta, k, alpha);
		
				Complex::new(-2.0 + guiding_space, free_space - loss)
			}).collect()
		},
		(Idx::Value(y), Idx::Free) => {
			(0..x_depht).map(|x| {
				let guiding_space = guiding_space(core, z, y, x, ydelta, k);
				let free_space = free_space(core, zdelta, ydelta, k);
				let loss = loss(core, ydelta, k, alpha);
		
				Complex::new(-2.0 + guiding_space, free_space - loss)
			}).collect()
		}
	}
}

fn guiding_space(core: &impl Core<3>, z: usize, y: usize, x: usize, delta: f64, k: f64) -> f64 {
	let n0 = core.get_n0();

	0.5*k.powf(2.0)*delta.powf(2.0)*(core.get_half_n(&[z, y, x], n0).powf(2.0)-n0.powf(2.0))
}

fn free_space(core: &impl Core<3>, zdelta: f64, delta: f64, k: f64) -> f64 {
	let n0 = core.get_n0();
	
	4.0*k*n0*delta.powf(2.0)/zdelta
}

fn loss(core: &impl Core<3>, delta: f64, k: f64, alpha: f64) -> f64 {
	let n0 = core.get_n0();

	k*n0*delta.powf(2.0)*alpha
}