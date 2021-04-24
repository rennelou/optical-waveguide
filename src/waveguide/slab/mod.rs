use crate::array::Array2d;

use super::*;
use super::core_waveguide::Core;
use super::eletric_field_2d;
use super::eletric_field_2d::EletricField2d;

use fp::list;
use fp::list::List;

pub struct Slab2d {
	pub grid: Array2d,

	s: List<List<Complex<f64>>>,
	q: List<List<Complex<f64>>>,
}

pub fn new(core: &impl Core, k: f64, alpha: f64) -> Slab2d {
    let grid = core.get_grid();

	let xdelta = grid.get_x().delta;
	let zdelta = grid.get_z().delta;
	let n0 = core.get_n0();

    let guiding_space = |x: f64, z: f64| k.powf(2.0)*xdelta.powf(2.0)*(core.get_half_n(x, z, n0).powf(2.0)-n0.powf(2.0));
    let free_space = || 4.0*k*n0*xdelta.powf(2.0)/zdelta;
    let loss = |_, _| 2.0*k*n0*xdelta.powf(2.0)*alpha;
    
    let s = grid.get_z().get_points().map(
        |z| grid.get_x().get_points().map(
            // okamoto 7.98
            |x| Complex::new(2.0 - guiding_space(x, z), free_space() + loss(x, z))
        ).collect()
    ).collect();
    
    let q = grid.get_z().get_points().map(
        |z| grid.get_x().get_points().map(
            // okamoto 7.99
            |x| Complex::new(-2.0 + guiding_space(x, z), free_space() - loss(x, z))
        ).collect()
    ).collect();
    
    Slab2d{ grid: grid.clone(), s, q }
}

pub fn fdmbpm(waveguide: &Slab2d, e_input: List<Complex<f64>>, boundary_codition: fn()->Complex<f64>) -> EletricField2d {
	
	let zsteps = waveguide.grid.get_z().steps;

	let es = (1usize..zsteps).fold(
		vec![e_input], 
		|result, i| {
			
			let last_es = fp::last_or_default(&result, list::empty());
			let last_q = waveguide.q[i-1].clone();
			
			let ds = get_ds(last_es, last_q);
			let new_es = insert_boundary_values(
				get_recurrence_form(get_alphas_betas(&waveguide.s[i], &ds, boundary_codition)),
				boundary_codition
			);

			return list::append(result, new_es);
		}
	);

	return eletric_field_2d::new(waveguide, es);
}

fn insert_boundary_values(es: List<Complex<f64>>, boundary_codition: fn()->Complex<f64>) -> List<Complex<f64>>{
	
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
