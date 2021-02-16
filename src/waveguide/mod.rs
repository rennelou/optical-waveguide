use crate::fp;	
use num::complex::Complex;
use fp::list;
pub mod slab;

const MINIMALSTEP: i64 = 5;

#[derive(Debug, Clone, Copy)]
struct Abc {
	a: Complex<f64>,
	b: Complex<f64>,
	c: Complex<f64>,
}

#[derive(Debug, Clone, Copy)]
struct AlphaBeta {
	alpha: Complex<f64>,
	beta: Complex<f64>,
}

impl AlphaBeta {
	
	pub fn empty() -> AlphaBeta {
		return AlphaBeta {
			alpha: zero(),
			beta: zero(),
		}
	}

}

fn get_recurrence_form(alpha_betas: Vec<AlphaBeta>) -> Vec<Complex<f64>> {
	
	return alpha_betas.iter().rev().fold(
		list::empty(),
		|es, alpha_beta| {
			
			let last_value = fp::unwrap_or_default(
				fp::head(&es), 
				one()
			);
			
			// okamoto 7.110
			let new_value= last_value * alpha_beta.alpha + alpha_beta.beta;
			
			return list::concat(list::new(new_value),es);
		}
	);
}

fn get_alphas_betas(abcs: Vec<Abc>, ds: Vec<Complex<f64>>) -> Vec<AlphaBeta> {
	
	return abcs.iter().enumerate().fold(
		list::empty(), 
		|alpha_betas, (i, abc)| {
		
			let last_alpha_beta = fp::unwrap_or_default(
				fp::last(&alpha_betas),
				AlphaBeta::empty()
			);
		
			let new_alpha_beta = AlphaBeta {
				// okamoto 7.112a
				alpha: abc.c / (abc.b - abc.a*last_alpha_beta.alpha),
				// okamoto 7.112b     		
				beta: (ds[i] + abc.a*last_alpha_beta.beta) / (abc.b - last_alpha_beta.alpha),
			};
			return list::append(alpha_betas, new_alpha_beta);
		}
	);
}

fn get_ds(es: &Vec<Complex<f64>>, qs: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
	
	if es.len() == qs.len() {
		return fp::init(&fp::tail(&qs)).iter().enumerate().fold(
			list::empty(), 
			|ds,(i, q)| {
				// okamoto 7.97
				let new_d = es[i]+q*es[i+1]+es[i+2];

				return list::append(ds, new_d);
			}
		)
	}

	return list::empty();
}

fn zero() -> Complex<f64> {
	return Complex::new(0.0, 0.0);
}

fn one() -> Complex<f64> {
	return Complex::new(1.0, 0.0);
}
