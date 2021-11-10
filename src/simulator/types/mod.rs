pub mod phasor;
pub mod simulation_result;

use crate::functional_types::Matrix;
use num::Complex;

pub type Phasor = Complex<f64>;

pub struct SimulationResult {
    pub intensity: Matrix<f64>,
    pub eletric_field: Matrix<f64>,
    pub shape: Vec<usize>,
    pub grid_steps: Vec<f64>,
    pub refractive_indexes: Vec<f64>
}