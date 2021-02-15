use crate::fp;	
use num::complex::Complex;
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
	let es: Vec<Complex<f64>> = alpha_betas.iter().rev().fold(vec![],
		|mut result, alpha_beta| {
			
			let last_value = if result.is_empty() {
				zero()
			} else {
				fp::last(&result).unwrap()
			};
			 
			// okamoto 7.110
			result.push(last_value * alpha_beta.alpha + alpha_beta.beta);
			
			return result;
		}
	);

	return es.iter().rev().cloned().collect();
}

fn get_alphas_betas(abcs: Vec<Abc>, ds: Vec<Complex<f64>>) -> Vec<AlphaBeta> {
	
	return abcs.iter().enumerate().fold(vec![], |mut alpha_betas, (i, abc)| {
			
			let last_alpha_beta = if alpha_betas.is_empty() {
				AlphaBeta::empty()
			} else {
				fp::last(&alpha_betas).unwrap()
			};
			
			alpha_betas.push(
				AlphaBeta {
					// okamoto 7.112a
					alpha: abc.c / (abc.b - abc.a*last_alpha_beta.alpha),
					// okamoto 7.112b     		
					beta: (ds[i] + abc.a*last_alpha_beta.beta) / (abc.b - last_alpha_beta.alpha),
				}
			);
			
			return alpha_betas;
		}
	);
}

fn get_ds(es: &Vec<Complex<f64>>, qs: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
	
	if es.len() == qs.len() {
		return fp::init(&fp::tail(&qs)).iter().enumerate().fold(vec![], |mut result,(i, q)| {
					// okamoto 7.97
					result.push(es[i]+q*es[i+1]+es[i+2]);

					return result;
			}
		)
	}

	return vec![];
}

fn zero() -> Complex<f64> {
	return Complex::new(0.0, 0.0);
}
