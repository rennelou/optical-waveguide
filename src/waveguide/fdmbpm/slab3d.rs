use crate::fp::matrix::Index;

use super::*;
use cores::Core;
use Phasor;
use fp::{comprehension, list};
use fp::Matrix;

pub fn run(core: &impl Core, k: f64, alpha: f64, e_input: Matrix<Phasor>, boundary_codition: fn()-> Phasor) -> EletricField {
	let shape = core.get_shape();
	let grid_steps = core.get_deltas().clone();
	let zdepht = shape[0];
	let ydepht = shape[1];
	let xdepht = shape[2];

	let (s, S, q, Q) = get_initialized_params_3d(core, k, alpha);

	let es = (1usize..zdepht).fold(
		vec![e_input], 
		|mut result, z| {
			
			let mut d_list = vec![];
			for x in 0..xdepht {
				let last_es= fp::last(result.iter()).unwrap().view(&[Index::Free, Index::Value(x)]);
				let last_Q = Q.view::<1>(&[Index::Value(z-1), Index::Free, Index::Value(x)]);
				
				// multiplicar d pelo fator para 3 dimensões
				let d = get_ds(last_es, last_Q);

				d_list = list::append(d_list, d);
			}
			let transposed_d_plane = matrix::zip(d_list);

			for y in 0..ydepht {
				let s_list = s.view::<1>(&[Index::Value(z), Index::Value(y), Index::Free]);
				let d_list = transposed_d_plane.view::<1>(&[Index::Free, Index::Value(y)]);

				let new_es = insert_boundary_values(
					get_recurrence_form(get_alphas_betas(s_list, d_list, boundary_codition)),
					boundary_codition
				);

				let shape = vec![new_es.len()];
				let new_es = matrix::new(new_es, &shape);

				result = list::append(result, new_es);
			}

			result
		}
	);

	let values = matrix::zip(es);
	return EletricField { values, grid_steps };
}

pub fn get_initialized_params_3d(core: &impl Core, k: f64, alpha: f64) 
-> (Matrix<Phasor>, Matrix<Phasor>, Matrix<Phasor>, Matrix<Phasor>) {
	let shape = core.get_shape();
	let mut shape_delta = shape.clone().into_iter().zip(core.get_deltas().clone().into_iter());
	let (zdepht, zdelta) = shape_delta.next().unwrap();
	let (ydepht, ydelta) = shape_delta.next().unwrap();
	let (xdepht, xdelta) = shape_delta.next().unwrap();
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

    let s = fp::new_3d(s_params(xdelta), shape) ;
    let S = fp::new_3d(s_params(ydelta), shape);
    let q = fp::new_3d(q_params(xdelta), shape);
    let Q = fp::new_3d(q_params(ydelta), shape);
    
    (s, S, q, Q)
}

fn insert_boundary_values(es: Vec<Phasor>, boundary_codition: fn() -> Phasor) -> Vec<Phasor>{
	
	let head = vec![{
		let es_head = fp::head_or_default(es.iter(), one());
		es_head*boundary_codition()
	}];
	let last = vec![{
		let es_last = fp::last_or_default(es.iter(), one());
		es_last*boundary_codition()
	}];
	
	return list::concat(list::concat(head, es),last);
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::{error::Error, f64::consts::PI};
	use ndarray::Array;

	#[test]
	fn slab2() -> Result<(), Box<dyn Error>> {
		let k0 = (2.0*PI)/1.55e-6_f64;

		let xdepht = 1024usize;
		let ydepht = 500usize;
		let total_depht = xdepht * ydepht;

    	let dx = 260e-6 * k0;
    	let xdelta = dx/(xdepht as f64);

		let ydelta = xdelta;
		//let dy = ydelta * (ydepht as f64);
		
    	let zdelta = 0.5e-6 * k0;
    	let dz = zdelta * 1000.0;

    	let position = dx/2.0;
    	let width = 35e-6 * k0;

    	let n0 = 3.0;
    	let n = 3.3;

    	let core = cores::rectilinear::new_2d(dx, xdelta, dz, zdelta, n, n0, position, width);
		
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