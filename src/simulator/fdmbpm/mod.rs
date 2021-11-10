pub mod slab;
pub mod grid;
pub mod beam;
pub mod boundary_codition;
pub mod cores;

use crate::tools::export;
use super::*;
use types::*;
use input::get_simulation;
use num::complex::Complex;

pub trait WaveguideSimulation {
	fn run(self) -> SimulationResult;
}

pub enum Waveguides {
	Bidimensional(slab::Slab<2,1>),
	Tridimensional(slab::Slab<3,2>)
}

impl WaveguideSimulation for Waveguides {
	fn run(self) -> SimulationResult {
		match self {
			Waveguides::Bidimensional(bidimensional_simulation) => {
				bidimensional_simulation.run()
			},
			Waveguides::Tridimensional(tridimensional_simulation) => {
				tridimensional_simulation.run()
			}
		}
	}
}

pub fn run(serialized: &str, output_name: &str) {
	export::hdf5(
		output_name,
		get_simulation(serialized).run()	
	)
}
