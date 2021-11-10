pub mod slab;
pub mod grid;
pub mod beam;
pub mod boundary_codition;
pub mod eletric_field;
pub mod cores;
mod types;

use super::*;
use types::*;
use input::get_simulation;
use num::complex::Complex;

pub trait WaveguideSimulation {
	fn run(self) -> Box<dyn SimulationResults>;
}

pub trait SimulationResults {
	fn export(&self, output_name: &str);
}

pub enum Waveguides {
	Bidimensional(slab::Slab<2,1>),
	Tridimensional(slab::Slab<3,2>)
}

impl WaveguideSimulation for Waveguides {
	fn run(self) -> Box<dyn SimulationResults>{
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
	get_simulation(serialized).run().export(output_name);
}
