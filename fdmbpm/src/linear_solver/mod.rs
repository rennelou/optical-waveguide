use num::Complex;
use itertools::izip;
use crate::fp::{self, list};

const COMPLEX_ZERO: Complex<f64> = Complex::new(0.0, 0.0);
const fn zero() -> &'static Complex<f64> {
	&COMPLEX_ZERO
}

const COMPLEX_ONE: Complex<f64> = Complex::new(1.0, 0.0);
const fn one() -> &'static Complex<f64> {
	&COMPLEX_ONE
}

pub fn try_solve_by_thomas(below_diag: Vec<Complex<f64>>, diag: Vec<Complex<f64>>, above_diag: Vec<Complex<f64>>, const_terms: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
	
	if below_diag.len() != diag.len() || above_diag.len() != diag.len() || const_terms.len() != diag.len() {
		panic!("all input vectors must have the sames lenghts")
	}
	
	if fp::head(below_diag.iter()).unwrap() != zero() {
		panic!("before_diagonal vector must init with 0")
	}

	if fp::last(above_diag.iter()).unwrap() != zero() {
		panic!("above_diagonal vector must ends with 0")
	}

	thomas(below_diag, diag, above_diag, const_terms)

}

fn thomas(below_diag: Vec<Complex<f64>>, diag: Vec<Complex<f64>>, above_diag: Vec<Complex<f64>>, const_terms: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
	
	let alpha_betas = row_echelon_form(below_diag, diag, above_diag, const_terms);
	
	alpha_betas.into_iter().rev().fold(
		vec![],
		|result, (alpha, beta)| {
			
			let last_value = fp::last_or_default(result.iter(), one());
			
			let new_value= last_value * alpha + beta;
			
			list::append(result, new_value)
		}
	).into_iter().rev().collect()
}

fn row_echelon_form(below_diag: Vec<Complex<f64>>, diag: Vec<Complex<f64>>, above_diag: Vec<Complex<f64>>, const_terms: Vec<Complex<f64>>) -> Vec<(Complex<f64>,Complex<f64>)> {
	
	izip!(&below_diag, &diag, &above_diag, &const_terms).fold(
		vec![],
		|alpha_beta, (a, d, b, c)| {
			let &(last_alpha, last_beta) = fp::last_or_default(alpha_beta.iter(), &(*zero(), *zero()));
			
			let alpha = a / (d - (b * last_alpha));
			let  beta = (c - (b * last_beta)) / (d - (b * last_alpha));

			list::append(alpha_beta, (alpha, beta))
		}
	)
}