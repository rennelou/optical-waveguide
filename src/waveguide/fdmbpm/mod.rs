use super::*;
use crate::fp;
use crate::fp::list;

pub mod slab2d;
//pub mod slab3d;

#[derive(Clone, Copy)]
struct AlphaBeta {
	alpha: Phasor,
	beta: Phasor,
}

const ALPHA_BETA_ZERO: AlphaBeta = AlphaBeta {
	alpha: *zero(),
	beta: *zero(),
};

impl AlphaBeta {
	
	const fn empty() -> &'static AlphaBeta {
		&ALPHA_BETA_ZERO
	}

}

fn get_recurrence_form(alpha_betas:  Vec<AlphaBeta>) -> Vec<Phasor> {
	
	return alpha_betas.into_iter().rev().fold(
		vec![],
		|es, alpha_beta| {
			
			let last_value = fp::head_or_default(es.iter(), one());
			
			// okamoto 7.110
			let new_value= last_value * alpha_beta.alpha + alpha_beta.beta;
			
			return list::concat(vec![new_value],es);
		}
	);
}

fn get_alphas_betas(ss: Vec<Phasor>, ds: Vec<Phasor>, boundary_codition: fn()->Phasor) -> Vec<AlphaBeta> {
	if ss.len() != ds.len() + 2 {
		panic!("ss array need has 2 more elements than ds array");
	}

	let len = ds.len();
	fp::middle(ss.iter()).zip(ds.iter()).enumerate().fold(
		vec![], 
		|alpha_betas, (i, (s, d))| {

			let last_value = fp::last_or_default(alpha_betas.iter(),AlphaBeta::empty());

			let new_value = if i == 0 {
				AlphaBeta {
					alpha: 1.0/(s-boundary_codition()),
					beta: d/(s-boundary_codition()) 
				}
			} else if i == len - 1 {
				AlphaBeta {
					alpha: *zero(),
					beta: (d + last_value.beta)/(s-boundary_codition()-last_value.alpha) 
				}
			} else {	
				AlphaBeta {
					alpha: 1.0 / (s - last_value.alpha), // okamoto 7.112a
					beta: (d + last_value.beta) / (s - last_value.alpha), // okamoto 7.112b
				}
			};

			return list::append(alpha_betas, new_value);
		}
	)
}

fn get_ds(es: Vec<Phasor>, qs: Vec<Phasor>) -> Vec<Phasor> {
	
	if es.len() == qs.len() {
		return fp::middle(qs.iter()).enumerate().map(
			// okamoto 7.97
			|(i, q)| es[i]+q*es[i+1]+es[i+2]
		).collect();
	}

	panic!("es array and qs array dosent have the same size");
}