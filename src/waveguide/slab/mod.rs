use super::*;
use core_waveguide::Core;
use Phasor;
use eletric_field_2d::EletricField2d;
use fp::comprehension;
use fp::list;
use fp::list::List;

pub fn fdmbpm_2d(core: &impl Core<2usize>, k: f64, alpha: f64, e_input: List<Phasor>, boundary_codition: fn()-> Phasor) -> EletricField2d {
	
	let [zsteps, xsteps] = core.get_shape();
	let [zdelta, xdelta]  = core.get_deltas();

	let (s, q) = get_initialized_params_2d(core, k, alpha);

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

	let shape = (zsteps, xsteps);
	let deltas = (zdelta, xdelta);
	return EletricField2d { es, shape, deltas };
}

pub fn get_initialized_params_2d(core: &impl Core<2usize>, k: f64, alpha: f64) -> (List<List<Phasor>>, List<List<Phasor>>) {
	let [zsteps, xsteps] = core.get_shape();
	let [zdelta, xdelta]  = core.get_deltas();
	let n0 = core.get_n0();

    let guiding_space = |x: f64, z: f64| k.powf(2.0)*xdelta.powf(2.0)*(core.get_half_n(x, z, n0).powf(2.0)-n0.powf(2.0));
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
		let es_head = fp::head_or_default(&es, one());
		es_head*boundary_codition()
	});
	let last = list::new({
		let es_last = fp::last_or_default(&es, one());
		es_last*boundary_codition()
	});
	
	return list::concat(list::concat(head, es),last);
}
