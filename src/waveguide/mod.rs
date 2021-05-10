use num::complex::Complex;
use crate::fp;	
use crate::fp::{list, List};

pub mod boundary_codition;
pub mod eletric_field;
pub mod slab;
pub mod cores;

pub type Phasor = Complex<f64>;

pub struct EletricField<const N: usize> {
    pub values: List<List<Phasor>>,
    pub shape: [usize;N],
    pub deltas: [f64;N]
}

#[derive(Clone, Copy)]
struct AlphaBeta {
	alpha: Phasor,
	beta: Phasor,
}

impl AlphaBeta {
	
	pub fn empty() -> AlphaBeta {
		return AlphaBeta {
			alpha: zero(),
			beta: zero(),
		}
	}

}

fn get_recurrence_form(alpha_betas: List<AlphaBeta>) -> List<Phasor> {
	
	return alpha_betas.into_iter().rev().fold(
		list::empty(),
		|es, alpha_beta| {
			
			let last_value = fp::head_or_default(&es, one());
			
			// okamoto 7.110
			let new_value= last_value * alpha_beta.alpha + alpha_beta.beta;
			
			return list::concat(list::new(new_value),es);
		}
	);
}

fn get_alphas_betas(ss: &List<Phasor>, ds: &List<Phasor>, boundary_codition: fn()->Phasor) -> List<AlphaBeta> {
	
	if ss.len() != ds.len() + 2 {
		panic!("ss array need has 2 more elements than ds array");
	}

	let cropped_s = &fp::body(ss);

	let s1 = fp::head_or_default(cropped_s, zero());
	let d1 = fp::head_or_default(ds, zero());
	
	let alpha_beta_one = AlphaBeta {
		alpha: 1.0/(s1-boundary_codition()),
		beta: d1/(s1-boundary_codition()) 
	};

	let alpha_betas = fp::body(cropped_s).into_iter().zip(fp::body(ds)).fold(
		fp::list::new(alpha_beta_one), 
		|alpha_betas, (s, d)| {
		
			let last_alpha_beta = fp::last_or_default(&alpha_betas,AlphaBeta::empty());
		
			let new_alpha_beta = AlphaBeta {
				// okamoto 7.112a
				alpha: 1.0 / (s - last_alpha_beta.alpha),
				// okamoto 7.112b     		
				beta: (d + last_alpha_beta.beta) / (s - last_alpha_beta.alpha),
			};
			return list::append(alpha_betas, new_alpha_beta);
		}
	);

	let sn = fp::last_or_default(cropped_s, zero());
	let dn = fp::last_or_default(ds, zero());
	let alpha_beta_n_less_one = fp::last_or_default(&alpha_betas,AlphaBeta::empty());

	let alpha_beta_n = AlphaBeta {
		alpha: zero(),
		beta: (dn + alpha_beta_n_less_one.beta) / (sn-boundary_codition()-alpha_beta_n_less_one.alpha) 
	};

	return list::append(alpha_betas, alpha_beta_n);
}

fn get_ds(es: List<Phasor>, qs: List<Phasor>) -> List<Phasor> {
	
	if es.len() == qs.len() {
		
		let cropped_qs = fp::body(&qs);
		
		return cropped_qs.into_iter().enumerate().fold(
			list::empty(), 
			|ds,(i, q)| {
				// okamoto 7.97
				let new_d = es[i]+q*es[i+1]+es[i+2];

				return list::append(ds, new_d);
			}
		)
	}

	panic!("es array and qs array dosent have the same size");
}

pub fn zero() -> Phasor {
	return Complex::new(0.0, 0.0);
}

pub fn one() -> Phasor {
	return Complex::new(1.0, 0.0);
}

pub fn to_phasor(x: f64) -> Phasor {
	return Complex::new(x, 0.0);
}
