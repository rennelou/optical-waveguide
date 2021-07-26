use super::*;
use Phasor;
use cores::Core;
use crate::fp::matrix;
use crate::fp::list;
use crate::waves;
use crate::waves::Gaussian;

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
	let grid_steps = core.get_deltas().to_vec();
	return EletricField { values, grid_steps };
}

fn get_s(core: &impl Core<2>, z: usize, beam: &Gaussian<1>) -> Vec<Phasor> {
	let k = beam.k;
	let alpha = beam.alpha;

	let [_, x_depht] = core.get_shape().clone();
	let &[zdelta, xdelta] = core.get_deltas();

	(0..x_depht).map(|x| {
		s(core, [z, x], zdelta, xdelta, k, alpha)
	}).collect()
}

fn get_q(core: &impl Core<2>, z: usize, beam: &Gaussian<1>) -> Vec<Phasor> {
	let k = beam.k;
	let alpha = beam.alpha;

	let [_, x_depht] = core.get_shape().clone();
	let &[zdelta, xdelta] = core.get_deltas();

	(0..x_depht).map(|x| {
		q(core, [z, x], zdelta, xdelta, k, alpha)
	}).collect()
}

fn guiding_space<const D: usize>(core: &impl Core<D>, position: [usize;D], delta: f64, k: f64) -> f64 {
	let n0 = core.get_n0();

	k.powf(2.0)*delta.powf(2.0)*(core.get_half_n(&position, n0).powf(2.0)-n0.powf(2.0))
}

fn free_space<const D: usize>(core: &impl Core<D>, zdelta: f64, delta: f64, k: f64) -> f64 {
	let n0 = core.get_n0();
	
	4.0*k*n0*delta.powf(2.0)/zdelta
}

fn loss<const D: usize>(core: &impl Core<D>, delta: f64, k: f64, alpha: f64) -> f64 {
	let n0 = core.get_n0();

	2.0*k*n0*delta.powf(2.0)*alpha
}

// Todo essas funções serão compartilhadas entre slab2d e slab3d
fn s<const D: usize>(core: &impl Core<D>, position: [usize;D], zdelta: f64, delta: f64, k: f64, alpha: f64) -> Phasor {
	let (guiding_space, free_space, loss) = slab_formulas(core, position, zdelta, delta, k, alpha);
	Complex::new(2.0 - guiding_space, free_space + loss)
}

fn q<const D: usize>(core: &impl Core<D>, position: [usize;D], zdelta: f64, delta: f64, k: f64, alpha: f64) -> Phasor {
	let (guiding_space, free_space, loss) = slab_formulas(core, position, zdelta, delta, k, alpha);
	Complex::new(-2.0 + guiding_space, free_space - loss)
}

fn slab_formulas<const D: usize>(core: &impl Core<D>, position: [usize;D], zdelta: f64, delta: f64, k: f64, alpha: f64) -> (f64, f64, f64) {
	let guiding_space = guiding_space(core, position, delta, k);
	let free_space = free_space(core, zdelta, delta, k);
	let loss = loss(core, delta, k, alpha);

	(guiding_space, free_space, loss)
}