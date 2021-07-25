use super::*;
use super::boundary_codition::Side;
use crate::fp::{self, matrix};
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

fn get_es(ss: Vec<Phasor>, ds: Vec<Phasor>, last_es: &Vec<Phasor>, boundary_codition: fn(s: Side, es: &Vec<Phasor>)->Phasor) -> Matrix<Phasor,1> {
	let left_boundaty = boundary_codition(Side::Left, &last_es);
	let right_boundaty = boundary_codition(Side::Right, &last_es);
	let values = insert_boundary_values(
		get_recurrence_form(get_alphas_betas(ss, ds, left_boundaty, right_boundaty)),
		boundary_codition
	);
	
	let shape = values.len();
	matrix::new(values, &[shape])
}

fn get_recurrence_form(alpha_betas:  Vec<AlphaBeta>) -> Vec<Phasor> {
	
	return alpha_betas.into_iter().rev().fold(
		vec![],
		|es, alpha_beta| {
			
			let last_value = fp::last_or_default(es.iter(), one());
			
			// okamoto 7.110
			let new_value= last_value * alpha_beta.alpha + alpha_beta.beta;
			
			list::append(es, new_value)
		}
	).into_iter().rev().collect();
}

fn get_alphas_betas(ss: Vec<Phasor>, ds: Vec<Phasor>, left_boundary: Phasor, right_boundary: Phasor) -> Vec<AlphaBeta> {
	if ss.len() != ds.len() + 2 {
		panic!("ss array need has 2 more elements than ds array");
	}

	let depht = ds.len();
	fp::middle(ss.iter()).zip(ds.iter()).enumerate().fold(
		vec![], 
		|alpha_betas, (i, (&s, d))| {

			let last_value = fp::last_or_default(alpha_betas.iter(),AlphaBeta::empty());

			let new_value = if i == 0 {
				AlphaBeta {
					alpha: 1.0/(s-left_boundary),
					beta: d/(s-left_boundary) 
				}
			} else if i == depht - 1 {
				AlphaBeta {
					alpha: *zero(),
					beta: (d + last_value.beta)/(s-right_boundary-last_value.alpha) 
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
fn get_ds(es: &Vec<Phasor>, qs: Vec<Phasor>) -> Vec<Phasor> {
	
	if es.len() == qs.len() {
		let es: Vec<_> = es.iter().map(|x|x).collect();
		let values = fp::middle(qs.iter()).enumerate().map(
			// okamoto 7.97
			|(i, q)| es[i]+q*es[i+1]+es[i+2]
		).collect();

		
		return values;
	}

	panic!("es array and qs array dosent have the same size");
}

fn insert_boundary_values(es: Vec<Phasor>, boundary_codition: fn(s: Side, es: &Vec<Phasor>) -> Phasor) -> Vec<Phasor> {

	let head = vec![{
		let es_head = fp::head_or_default(es.iter(), one());
		es_head*boundary_codition(Side::Left, &es)
	}];
	let last = vec![{
		let es_last = fp::last_or_default(es.iter(), one());
		es_last*boundary_codition(Side::Right, &es)
	}];
	
	let values = list::concat(list::concat(head, es),last);

	values
}