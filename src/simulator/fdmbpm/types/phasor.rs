use super::*;

const PHASOR_ZERO: Phasor = Complex::new(0.0, 0.0);
pub const fn zero() -> &'static Phasor {
	&PHASOR_ZERO
}

const PHASOR_ONE: Phasor = Complex::new(1.0, 0.0);
pub const fn one() -> &'static Phasor {
	&PHASOR_ONE
}