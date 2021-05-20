use super::*;
use crate::fp::{self, matrix};
use crate::fp::list;
use crate::fp::matrix::MatrixView;

pub mod slab2d;
pub mod slab3d;

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

fn get_es(ss: MatrixView<Phasor, 1usize>, ds: MatrixView<Phasor, 1usize>, boundary_codition: fn()->Phasor) -> Matrix<Phasor> {
	insert_boundary_values(
		get_recurrence_form(get_alphas_betas(ss, ds, boundary_codition)),
		boundary_codition
	)
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

fn get_alphas_betas(ss: MatrixView<Phasor, 1usize>, ds: MatrixView<Phasor, 1usize>, boundary_codition: fn()->Phasor) -> Vec<AlphaBeta> {
	if ss.depht() != ds.depht() + 2 {
		panic!("ss array need has 2 more elements than ds array");
	}

	let depht = ds.depht();
	fp::middle(ss.iter()).zip(ds.iter()).enumerate().fold(
		vec![], 
		|alpha_betas, (i, (&s, d))| {

			let last_value = fp::last_or_default(alpha_betas.iter(),AlphaBeta::empty());

			let new_value = if i == 0 {
				AlphaBeta {
					alpha: 1.0/(s-boundary_codition()),
					beta: d/(s-boundary_codition()) 
				}
			} else if i == depht - 1 {
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

// TODO: lembrar de multiplicar pelo fator usado em 3 dimensões
fn get_ds(es: MatrixView<Phasor, 1usize>, qs: MatrixView<Phasor, 1usize>) -> Matrix<Phasor> {
	
	if es.depht() == qs.depht() {
		let es: Vec<_> = es.iter().map(|x|x).collect();
		let values: Vec<_> = fp::middle(qs.iter()).enumerate().map(
			// okamoto 7.97
			|(i, q)| es[i]+q*es[i+1]+es[i+2]
		).collect();

		let shape = vec![values.len()];
		return matrix::new(values, &shape);
	}

	panic!("es array and qs array dosent have the same size");
}

fn insert_boundary_values(es: Vec<Phasor>, boundary_codition: fn() -> Phasor) -> Matrix<Phasor> {
	
	let head = vec![{
		let es_head = fp::head_or_default(es.iter(), one());
		es_head*boundary_codition()
	}];
	let last = vec![{
		let es_last = fp::last_or_default(es.iter(), one());
		es_last*boundary_codition()
	}];
	
	let values = list::concat(list::concat(head, es),last);

	let shape = vec![values.len()];
	matrix::new(values, &shape)
}