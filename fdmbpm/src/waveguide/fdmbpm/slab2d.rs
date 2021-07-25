use super::*;
use Phasor;
use cores::Core;
use crate::fp::matrix;
use crate::fp::list;
use crate::waves;
use crate::waves::Gaussian;

// e_input precisa ser unidimensional e nada no codigo garante que não é
// to_do criar alguma garantia no codigo de e_input ser unidimensional
pub fn run(core: &impl Core<2>, beam: Gaussian<1>, boundary_codition: fn(s: Side, es: &Vec<Phasor>)-> Phasor) -> EletricField<2> {
	let shape = core.get_shape();

	let zsteps = shape[0];

	let e_input = waves::input(&[shape[1]], &[core.get_deltas()[1]], &beam.center, beam.amplitude, beam.width);

	let es = (1usize..zsteps).fold( 
		vec![e_input],
		|acc, z| {
			
			let last_es= fp::last(acc.iter()).unwrap().raw();
			
			let last_q = get_q(core, z-1, &beam);
			let last_s = get_s(core, z-1, &beam);

			let ds = get_ds(last_es, last_q);

			let new_es = get_es(last_s, ds, last_es, boundary_codition);

			list::append(acc, new_es)
		}
	);

	let values = matrix::new_from_vec(es);
	let &grid_steps = core.get_deltas();
	return EletricField { values, grid_steps };
}

fn get_s(core: &impl Core<2>, z: usize, beam: &Gaussian<1>) -> Vec<Phasor> {
	let [_, x_depht] = core.get_shape().clone();
	let k = beam.k;
	let alpha = beam.alpha;

	(0..x_depht).map(|x| {
		let guiding_space = guiding_space(core, z, x, k);
		let free_space = free_space(core, k);
		let loss = loss(core, k, alpha);

		Complex::new(2.0 - guiding_space, free_space + loss)
	}).collect()
}

fn get_q(core: &impl Core<2>, z: usize, beam: &Gaussian<1>) -> Vec<Phasor> {
	let [_, x_depht] = core.get_shape().clone();
	let k = beam.k;
	let alpha = beam.alpha;

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