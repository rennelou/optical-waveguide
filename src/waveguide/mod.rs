use num::complex::Complex;	
use crate::fp::List;

pub mod fdmbpm;
pub mod boundary_codition;
pub mod eletric_field;
pub mod cores;

pub type Phasor = Complex<f64>;

pub struct EletricField<const N: usize> {
    pub values: List<Phasor>,
    pub shape: [usize;N],
    pub deltas: [f64;N]
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
