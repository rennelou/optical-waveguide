use super::*;
use cores::Core;
use Phasor;
use fp::{comprehension, list, List};
use fp::{matrix, Matrix};

pub fn run(core: &impl Core, k: f64, alpha: f64, e_input: Matrix<Phasor>, boundary_codition: fn()-> Phasor) -> EletricField {
	let shape = core.get_shape();
	let deltas = core.get_deltas();
	let zsteps = shape[0];

	let (s, S, q, Q) = get_initialized_params_3d(core, k, alpha);

	let es = (1usize..zsteps).fold(
		vec![e_input], 
		|result, i| {
			
			let last_es = fp::last_or_default(&result, list::empty());
			let last_q = q[i-1].clone();
			
			let ds = get_ds(last_es, last_q);
			let new_es = insert_boundary_values(
				get_recurrence_form(get_alphas_betas(&s[i], &ds, boundary_codition)),
				boundary_codition
			);

			return list::append(result, new_es);
		}
	);

	let values = flat(es);
	return EletricField { values, shape, deltas };
}

pub fn get_initialized_params_3d(core: &impl Core, k: f64, alpha: f64) 
-> (Matrix<Phasor>, Matrix<Phasor>, Matrix<Phasor>, Matrix<Phasor>) {
	let shape = core.get_shape();
	let mut shape_delta = shape.clone().into_iter().zip(core.get_deltas().clone().into_iter());
	let (zsteps, zdelta) = shape_delta.next().unwrap();
	let (ysteps, ydelta) = shape_delta.next().unwrap();
	let (xsteps, xdelta) = shape_delta.next().unwrap();
	let n0 = core.get_n0();

    let guiding_space = |x: f64, y:f64, z: f64, delta: f64| k.powf(2.0)*delta.powf(2.0)*(core.get_half_n(z,0.0, x, n0).powf(2.0)-n0.powf(2.0));
    let free_space = |delta: f64| 4.0*k*n0*delta.powf(2.0)/zdelta;
    let loss = |_, _, _, delta: f64| 2.0*k*n0*delta.powf(2.0)*alpha;
    
	let s_params = |delta: f64| -> List<List<List<Phasor>>> {
		comprehension::arange(zsteps, zdelta).map(|z| 
        	comprehension::arange(ysteps, ydelta).map(|y| 
            	comprehension::arange(xsteps, xdelta).map(|x| 
                	Complex::new(2.0 - guiding_space(x, y, z, delta), free_space(delta) + loss(x, y, z, delta))
            	).collect()
			).collect()
    	).collect()
	};

	let q_params = |delta: f64| -> List<List<List<Phasor>>> {
		comprehension::arange(zsteps, zdelta).map(|z|
			comprehension::arange(ysteps, ydelta).map(|y|
				comprehension::arange(xsteps, xdelta).map(|x| 
					Complex::new(-2.0 + guiding_space(x, y, z, delta), free_space(delta) - loss(x, y, z, delta))
				).collect()
			).collect()
		).collect()
	};

    let s = matrix::new_3d(s_params(xdelta), shape) ;
    let S = matrix::new_3d(s_params(ydelta), shape);
    let q = matrix::new_3d(q_params(xdelta), shape);
    let Q = matrix::new_3d(q_params(ydelta), shape);
    
    (s, S, q, Q)
}

fn insert_boundary_values(es: List<Phasor>, boundary_codition: fn() -> Phasor) -> List<Phasor>{
	
	let head = list::new({
		let es_head = fp::head_or_default(&es, one());
		es_head*boundary_codition()
	});
	let last = list::new({
		let es_last = fp::last_or_default(&es, one());
		es_last*boundary_codition()
	});
	
	return list::concat(list::concat(head, es),last);
}