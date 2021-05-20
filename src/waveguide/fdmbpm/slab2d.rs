use super::*;
use Phasor;
use cores::Core;
use crate::fp::matrix::{self, Idx};
use crate::fp::list;

pub fn run(core: &impl Core<2>, k: f64, alpha: f64, e_input: Matrix<Phasor>, boundary_codition: fn()-> Phasor) -> EletricField {
	let shape = core.get_shape().clone();
	let grid_steps = core.get_deltas().to_vec();
	let zsteps = shape[0];

	let (s, q) = get_initialized_params_2d(core, k, alpha);

	let es = (1usize..zsteps).fold(
		vec![e_input], 
		|result, i| {
			
			let last_es= fp::last(result.iter()).unwrap().view(&[Idx::Free]);
			let last_q = q.view(&[Idx::Value(i-1), Idx::Free]);

			let s_list = s.view(&[Idx::Value(i-1), Idx::Free]);

			let ds = get_ds(last_es, last_q);
			let d_list = ds.view(&[Idx::Free]);

			let new_es = get_es(s_list, d_list, boundary_codition);

			return list::append(result, new_es);
		}
	);

	let values = matrix::zip(es);
	return EletricField { values, grid_steps };
}

pub fn get_initialized_params_2d(core: &impl Core<2>, k: f64, alpha: f64) -> (Matrix<Phasor>, Matrix<Phasor>) {
	let shape = core.get_shape().clone();
	let [zdelta, xdelta] = core.get_deltas().clone();
	
	let n0 = core.get_n0();

    let guiding_space = |position: &[usize]| k.powf(2.0)*xdelta.powf(2.0)*(core.get_half_n(position, n0).powf(2.0)-n0.powf(2.0));
    let free_space = || 4.0*k*n0*xdelta.powf(2.0)/zdelta;
    let loss = || 2.0*k*n0*xdelta.powf(2.0)*alpha;
    
    let (s,q) = (0..shape.iter().product()).map(|id| {
        let position = matrix::id_to_position(id, &shape);
		let position_slice = position.as_slice();
        (
			// okamoto 7.98
			Complex::new(2.0 - guiding_space(&position_slice), free_space() + loss()),
			// okamoto 7.99
			Complex::new(-2.0 + guiding_space(&position_slice), free_space() - loss())
		)

	}).unzip();
    
    (matrix::new(s,&shape.to_vec()), matrix::new(q, &shape.to_vec()))
}