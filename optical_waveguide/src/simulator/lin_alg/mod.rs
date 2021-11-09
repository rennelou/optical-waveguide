use super::fp;
use num::Complex;

pub mod thomas;

const COMPLEX_ZERO: Complex<f64> = Complex::new(0.0, 0.0);
const fn zero() -> &'static Complex<f64> {
	&COMPLEX_ZERO
}

const COMPLEX_ONE: Complex<f64> = Complex::new(1.0, 0.0);
const fn one() -> &'static Complex<f64> {
	&COMPLEX_ONE
}

pub struct DiagonalMatrix {
	pub below_diag: Vec<Complex<f64>>,
    pub diag: Vec<Complex<f64>>,
    pub above_diag:Vec<Complex<f64>>,

    pub lenght: usize
}

pub fn diagonal_matrix(below_diag: Vec<Complex<f64>>, diag: Vec<Complex<f64>>, above_diag: Vec<Complex<f64>>) -> DiagonalMatrix {
    if below_diag.len() != diag.len() || above_diag.len() != diag.len() {
		panic!("all input vectors must have the sames lenghts")
	}

    let lenght = below_diag.len();
	
	if fp::head(below_diag.iter()).unwrap() != zero() {
		panic!("before_diagonal vector must init with 0")
	}

	if fp::last(above_diag.iter()).unwrap() != zero() {
		panic!("above_diagonal vector must ends with 0")
	}

    DiagonalMatrix { below_diag, diag, above_diag, lenght }
}