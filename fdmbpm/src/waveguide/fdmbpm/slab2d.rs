use super::*;
use Phasor;
use cores::Core;
use crate::fp::matrix;
use crate::fp::list;

pub fn run(core: &impl Core<2>, k: f64, alpha: f64, e_input: Matrix<Phasor>, boundary_codition: fn(s: Side, es: &Vec<Phasor>)-> Phasor) -> EletricField {
	let shape = core.get_shape().clone();
	let grid_steps = core.get_deltas().to_vec();
	let zsteps = shape[0];

	// Por enquanto porque so tem uma dimensao
	let es_array = e_input.raw().clone();

	let mut es = vec![es_array];
	for z in 1usize..zsteps {
			
		let last_es= fp::last(es.iter()).unwrap();
		
		let last_q = get_q(core, z-1, k, alpha);
		let last_s = get_s(core, z-1, k, alpha);

		let ds = get_ds(last_es, last_q);

		let new_es = get_es(last_s, ds, last_es, boundary_codition);

		es = list::append(es, new_es);
	}

	let values = matrix::new(es.into_iter().flatten().collect(), &shape.to_vec());
	return EletricField { values, grid_steps };
}

fn get_s(core: &impl Core<2>, z: usize, k: f64, alpha: f64) -> Vec<Phasor> {
	let [_, x_depht] = core.get_shape().clone();
	
	(0..x_depht).map(|x| {
		let guiding_space = guiding_space(core, z, x, k);
		let free_space = free_space(core, k);
		let loss = loss(core, k, alpha);

		Complex::new(2.0 - guiding_space, free_space + loss)
	}).collect()
}

fn get_q(core: &impl Core<2>, z: usize, k: f64, alpha: f64) -> Vec<Phasor> {
	let [_, x_depht] = core.get_shape().clone();
	
	(0..x_depht).map(|x| {
		let guiding_space = guiding_space(core, z, x, k);
		let free_space = free_space(core, k);
		let loss = loss(core, k, alpha);

		Complex::new(-2.0 + guiding_space, free_space - loss)
	}).collect()
}

fn guiding_space(core: &impl Core<2>, z: usize, x: usize, k: f64) -> f64 {
	let [_, xdelta] = core.get_deltas();
	let n0 = core.get_n0();

	k.powf(2.0)*xdelta.powf(2.0)*(core.get_half_n(&[z, x], n0).powf(2.0)-n0.powf(2.0))
}

fn free_space(core: &impl Core<2>, k: f64) -> f64 {
	let [zdelta, xdelta] = core.get_deltas();
	let n0 = core.get_n0();
	
	4.0*k*n0*xdelta.powf(2.0)/zdelta
}

fn loss(core: &impl Core<2>, k: f64, alpha: f64) -> f64 {
	let [_, xdelta] = core.get_deltas();
	let n0 = core.get_n0();

	2.0*k*n0*xdelta.powf(2.0)*alpha
}