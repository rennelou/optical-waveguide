use crate::fp::matrix::Idx;

use super::*;
use cores::Core;
use Phasor;
use fp::{comprehension, list};
use fp::Matrix;

pub fn run(core: &impl Core<3>, k: f64, alpha: f64, e_input: Matrix<Phasor>, boundary_codition: fn()-> Phasor) -> EletricField {
	let grid_steps = core.get_deltas().to_vec();
	let [zdepht, ydepht, xdepht] = core.get_shape().clone();

	let (s, S, q, Q) = get_initialized_params_3d(core, k, alpha);

	let es = (1usize..zdepht).fold(
		vec![e_input], 
		|result, z| {
			
			let mut d_list = vec![];
			for x in 1..(xdepht-1) {
				let last_es= fp::last(result.iter()).unwrap().view(&[Idx::Free, Idx::Value(x)]);
				let last_Q = Q.view(&[Idx::Value(z-1), Idx::Free, Idx::Value(x)]);
				
				// multiplicar d pelo fator para 3 dimensões
				let d = get_ds(last_es, last_Q);

				d_list = list::append(d_list, d);
			}
			let transposed_d_plane = matrix::zip(d_list);

			let mut es_list = vec![];
			for y in 1..(ydepht-1) {
				let s_list = s.view(&[Idx::Value(z-1), Idx::Value(y), Idx::Free]);
				let d_list = transposed_d_plane.view(&[Idx::Free, Idx::Value(y-1)]);

				let new_es = insert_boundary_values(
					get_recurrence_form(get_alphas_betas(s_list, d_list, boundary_codition)),
					boundary_codition
				);

				es_list = list::append(es_list, new_es);
			}
			let es_intermediate = matrix::zip(es_list);
			
//----------------------- segunda parte -----------------------------------------------

			let mut h_list = vec![];
			for y in 1..(ydepht-1) {
				let last_es = es_intermediate.view(&[Idx::Value(y-1), Idx::Free]);
				let last_q = q.view(&[Idx::Value(z-1), Idx::Value(y), Idx::Free]);

				// multiplicar d pelo fator para 3 dimensões
				let h = get_ds(last_es, last_q);
				h_list = list::append(h_list, h);
			}
			let h_plane = matrix::zip(h_list);
			
			let mut es_list = vec![];
			for x in 1..(xdepht-1) {
				let S_list = S.view(&[Idx::Value(z-1), Idx::Free, Idx::Value(x)]);
				let h_list = h_plane.view(&[Idx::Free, Idx::Value(x-1)]);

				let new_es = insert_boundary_values(
					get_recurrence_form(get_alphas_betas(S_list, h_list, boundary_codition)),
					boundary_codition
				);

				es_list = list::append(es_list, new_es);
			}
			let es_transposed = matrix::zip(es_list);
			
			let mut es_list = vec![];
			for y in 0..ydepht {
				let es_to_insert_boundary_x = es_transposed.view::<1>(&[Idx::Free, Idx::Value(y)]).iter().cloned().collect();

				let new_es = insert_boundary_values(es_to_insert_boundary_x, boundary_codition);
				es_list = list::append(es_list, new_es);
			}
			let es = matrix::zip(es_list);

			list::append(result, es)
		}
	);

	let values = matrix::zip(es);
	return EletricField { values, grid_steps };
}

pub fn get_initialized_params_3d(core: &impl Core<3>, k: f64, alpha: f64) 
-> (Matrix<Phasor>, Matrix<Phasor>, Matrix<Phasor>, Matrix<Phasor>) {
	let [zdepht, ydepht, xdepht] = core.get_shape().clone();
	let [zdelta, ydelta, xdelta] = core.get_deltas().clone();
	let n0 = core.get_n0();

    let guiding_space = |x: f64, y:f64, z: f64, delta: f64| k.powf(2.0)*delta.powf(2.0)*(core.get_half_n(z, y, x, n0).powf(2.0)-n0.powf(2.0));
    let free_space = |delta: f64| 4.0*k*n0*delta.powf(2.0)/zdelta;
    let loss = |_, _, _, delta: f64| 2.0*k*n0*delta.powf(2.0)*alpha;
    
	let s_params = |delta: f64| -> Vec<Vec<Vec<Phasor>>> {
		comprehension::arange(zdepht, zdelta).map(|z| 
        	comprehension::arange(ydepht, ydelta).map(|y| 
            	comprehension::arange(xdepht, xdelta).map(|x| 
                	Complex::new(2.0 - guiding_space(x, y, z, delta), free_space(delta) + loss(x, y, z, delta))
            	).collect()
			).collect()
    	).collect()
	};

	let q_params = |delta: f64| -> Vec<Vec<Vec<Phasor>>> {
		comprehension::arange(zdepht, zdelta).map(|z|
			comprehension::arange(ydepht, ydelta).map(|y|
				comprehension::arange(xdepht, xdelta).map(|x| 
					Complex::new(-2.0 + guiding_space(x, y, z, delta), free_space(delta) - loss(x, y, z, delta))
				).collect()
			).collect()
		).collect()
	};

	let shape = core.get_shape().to_vec();
    let s = fp::new_3d(s_params(xdelta), &shape) ;
    let S = fp::new_3d(s_params(ydelta), &shape);
    let q = fp::new_3d(q_params(xdelta), &shape);
    let Q = fp::new_3d(q_params(ydelta), &shape);
    
    (s, S, q, Q)
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::{error::Error, f64::consts::PI};
	use ndarray::Array;

	#[test]
	fn slab2() -> Result<(), Box<dyn Error>> {
		let k0 = (2.0*PI)/1.55e-6_f64;

		let xdepht = 124usize;
		let ydepht = 50usize;
		let total_depht = xdepht * ydepht;

    	let dx = 260e-6 * k0;
    	let xdelta = dx/(xdepht as f64);

		let ydelta = xdelta;
		let dy = ydelta * (ydepht as f64);
		
    	let zdelta = 0.5e-6 * k0;
    	let dz = zdelta * 100.0;

    	let position = dx/2.0;
    	let width = 35e-6 * k0;

    	let n0 = 3.0;
    	let n = 3.3;

    	let core = cores::rectilinear::new_3d(dx, xdelta, dy, ydelta, dz, zdelta, n, n0, position, width);
		
    	//let p = 200.0;
    	//let eta = 120.0 * PI; // eta usa eps e mi do meio
    	//let w = 10e-6 * k0;
    	//let e0 = p*eta / (w.powf(2.0)*PI);
    	let input = matrix::new(vec![Complex::new(1.0, 1.0);total_depht], &vec![ydepht, xdepht]);

    	let e = fdmbpm::slab3d::run(&core, 1.0, 0.0, input, boundary_codition::dirichlet);
		// para gerar seria so exportar e -- export::hdf5("example.h5", &e);

		let intensity = e.get_intensity();
    	let _array = Array::from_shape_vec(e.shape().clone(), intensity)?;

    	//let file = hdf5::File::open("slab.h5")?;
		//let dir = file.group("dir")?;
		//let values = dir.dataset("intensity")?;

		//assert_eq!(values.read_dyn::<f64>()?, array);

		Ok(())
   	}
}