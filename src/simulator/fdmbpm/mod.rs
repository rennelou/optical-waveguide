pub mod slab;
pub mod grid;
pub mod beam;
pub mod boundary_codition;
pub mod eletric_field;
pub mod cores;

use super::*;
use super::input::get_simulation;

use num::complex::Complex;

pub type Phasor = Complex<f64>;

pub enum WaveguideSimulation {
	Bidimensional(slab::Slab<2,1>),
	Tridimensional(slab::Slab<3,2>)
}

const PHASOR_ZERO: Phasor = Complex::new(0.0, 0.0);
const fn zero() -> &'static Phasor {
	&PHASOR_ZERO
}

const PHASOR_ONE: Phasor = Complex::new(1.0, 0.0);
const fn one() -> &'static Phasor {
	&PHASOR_ONE
}

pub fn run(serialized: &str, output_name: &str) {
	match get_simulation(serialized) {
		WaveguideSimulation::Bidimensional(bidimensional_simulation) => {
			bidimensional_simulation.run().export(output_name);
		},
		WaveguideSimulation::Tridimensional(tridimensional_simulation) => {
			tridimensional_simulation.run().export(output_name);
		}
	}
}

pub fn to_phasor(x: f64) -> Phasor {
	return Complex::new(x, 0.0);
}
