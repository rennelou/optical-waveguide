use super::*;
use itertools::izip;

// m*x = const_terms
pub fn try_solve(m: DiagonalMatrix, const_terms: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
	if m.lenght == const_terms.len() {
        solve(m.below_diag, m.diag, m.above_diag, const_terms)
    } else {
        panic!("constant terms must have the same lenght that diagonal matrix")
    }
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
        
        let m = diagonal_matrix(below_diag, diag, above_diag);

        let expected = vec![cplx(1.0), cplx(2.0), cplx(3.0), cplx(4.0), cplx(5.0)];
        let result = try_solve(m, const_terms);
        assert!(izip!(&expected,&result).all( |(e, r)| (e-r).norm() < 1e-6 ));
    }

    fn cplx(x: f64) -> Complex<f64> {
        Complex::new(x, 0.0)
    }
}