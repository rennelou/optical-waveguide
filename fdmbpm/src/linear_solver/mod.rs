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