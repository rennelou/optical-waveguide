use super::*;
use Phasor;
use cores::Core;
use crate::fp;
use crate::fp::{comprehension, list, List};

static PHASOR_EMPTY_LIST: List<Phasor> = vec![];

pub fn run(core: &impl Core, k: f64, alpha: f64, e_input: List<Phasor>, boundary_codition: fn()-> Phasor) -> EletricField {
	let shape = core.get_shape().clone();
	let deltas = core.get_deltas().clone();
	let zsteps = shape[0];

	let (s, q) = get_initialized_params_2d(core, k, alpha);

	let es = (1usize..zsteps).fold(
		vec![e_input], 
		|result, i| {
			
			let last_es = fp::last_or_default(result.iter(), &PHASOR_EMPTY_LIST);
			let last_q = &q[i-1];
			
			let ds = get_ds(last_es, last_q);
			let new_es = insert_boundary_values(
				get_recurrence_form(get_alphas_betas(&s[i], &ds, boundary_codition)),
				boundary_codition
			);

			return list::append(result, new_es);
		}
	);

	let values = es.into_iter().flatten().collect::<List<Phasor>>();
	return EletricField { values, shape, deltas };
}

pub fn get_initialized_params_2d(core: &impl Core, k: f64, alpha: f64) -> (List<List<Phasor>>, List<List<Phasor>>) {
	let mut shape_delta = core.get_shape().clone().into_iter().zip(core.get_deltas().clone().into_iter());
	let (zsteps, zdelta) = shape_delta.next().unwrap();
	let (xsteps, xdelta) = shape_delta.next().unwrap();
	
	let n0 = core.get_n0();

    let guiding_space = |x: f64, z: f64| k.powf(2.0)*xdelta.powf(2.0)*(core.get_half_n(z,0.0, x, n0).powf(2.0)-n0.powf(2.0));
    let free_space = || 4.0*k*n0*xdelta.powf(2.0)/zdelta;
    let loss = |_, _| 2.0*k*n0*xdelta.powf(2.0)*alpha;
    
    let s = comprehension::arange(zsteps, zdelta).map(
        |z| comprehension::arange(xsteps, xdelta).map(
            
			// okamoto 7.98
            |x| Complex::new(2.0 - guiding_space(x, z), free_space() + loss(x, z))
        
		).collect()
    ).collect();
    
    let q = comprehension::arange(zsteps, zdelta).map(
        |z| comprehension::arange(xsteps, xdelta).map(
            
			// okamoto 7.99
            |x| Complex::new(-2.0 + guiding_space(x, z), free_space() - loss(x, z))
        
		).collect()
    ).collect();
    
    (s, q)
}

fn insert_boundary_values(es: List<Phasor>, boundary_codition: fn() -> Phasor) -> List<Phasor>{
	
	let head = list::new({
		let es_head = fp::head_or_default(es.iter(), one());
		es_head*boundary_codition()
	});
	let last = list::new({
		let es_last = fp::last_or_default(es.iter(), one());
		es_last*boundary_codition()
	});
	
	return list::concat(list::concat(head, es),last);
}