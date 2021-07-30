use num::complex::Complex;	
use crate::fp::Matrix;

pub mod slab;
pub mod beam;
pub mod boundary_codition;
pub mod eletric_field;
pub mod cores;

pub type Phasor = Complex<f64>;

const PHASOR_ZERO: Phasor = Complex::new(0.0, 0.0);
const fn zero() -> &'static Phasor {
	&PHASOR_ZERO
}

const PHASOR_ONE: Phasor = Complex::new(1.0, 0.0);
const fn one() -> &'static Phasor {
	&PHASOR_ONE
}

pub fn to_phasor(x: f64) -> Phasor {
	return Complex::new(x, 0.0);
}
