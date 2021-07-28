use super::*;
use itertools::izip;
use crate::fp::{self, list};

pub fn try_solve(below_diag: Vec<Complex<f64>>, diag: Vec<Complex<f64>>, above_diag: Vec<Complex<f64>>, const_terms: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
	
	if below_diag.len() != diag.len() || above_diag.len() != diag.len() || const_terms.len() != diag.len() {
		panic!("all input vectors must have the sames lenghts")
	}
	
	if fp::head(below_diag.iter()).unwrap() != zero() {
		panic!("before_diagonal vector must init with 0")
	}

	if fp::last(above_diag.iter()).unwrap() != zero() {
		panic!("above_diagonal vector must ends with 0")
	}

	solve(below_diag, diag, above_diag, const_terms)

}

fn solve(below_diag: Vec<Complex<f64>>, diag: Vec<Complex<f64>>, above_diag: Vec<Complex<f64>>, const_terms: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
	
	let alpha_betas = row_echelon_form(below_diag, diag, above_diag, const_terms);
	
	alpha_betas.into_iter().rev().fold(
		vec![],
		|result, (alpha, beta)| {
			
			let last_value = fp::last_or_default(result.iter(), one());
			
			let new_value= beta - (last_value * alpha);
			
			list::append(result, new_value)
		}
	).into_iter().rev().collect()
}

fn row_echelon_form(below_diag: Vec<Complex<f64>>, diag: Vec<Complex<f64>>, above_diag: Vec<Complex<f64>>, const_terms: Vec<Complex<f64>>) -> Vec<(Complex<f64>,Complex<f64>)> {
	
	izip!(&below_diag, &diag, &above_diag, &const_terms).fold(
		vec![],
		|alpha_beta, (b, d, a, c)| {
			let &(last_alpha, last_beta) = fp::last_or_default(alpha_beta.iter(), &(*zero(), *zero()));
			
			let alpha = a / (d - (b * last_alpha));
			let beta = (c - (b * last_beta)) / (d - (b * last_alpha));

			list::append(alpha_beta, (alpha, beta))
		}
	)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let below_diag = vec![cplx(0.0), cplx(-1.0), cplx(-1.0), cplx(-1.0), cplx(-1.0)];
        let diag = vec![cplx(2.0), cplx(2.0), cplx(2.0), cplx(2.0), cplx(1.0)];
        let above_diag = vec![cplx(-1.0), cplx(-1.0), cplx(-1.0), cplx(-1.0), cplx(0.0)];
        let const_terms = vec![cplx(0.0), cplx(0.0), cplx(0.0), cplx(0.0), cplx(1.0)];
        
        let expected = vec![cplx(1.0), cplx(2.0), cplx(3.0), cplx(4.0), cplx(5.0)];
        let result = try_solve(below_diag, diag, above_diag, const_terms);
        assert!(izip!(&expected,&result).all( |(e, r)| (e-r).norm() < 1e-6 ));
    }

    fn cplx(x: f64) -> Complex<f64> {
        Complex::new(x, 0.0)
    }
}