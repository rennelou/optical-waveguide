use crate::functional_types::matrix;
use super::*;

pub fn new(values: Matrix<Phasor>, grid_steps: Vec<f64>, refractive_indexes: Vec<f64>) -> SimulationResult {
    
	let shape = values.shape().to_vec();
	let eletric_field = get_eletric_fields(&values);
	let intensity = get_intensity(&values);
	
	SimulationResult { shape, intensity, eletric_field, grid_steps, refractive_indexes }
}

fn get_eletric_fields(values: &Matrix<Phasor>) -> Matrix<f64>  {
	let new_values = values.raw().iter().map(|p|{
		let (r, _theta) = p.clone().to_polar();

		r
	}).collect();

	matrix::new(new_values, values.shape())
}

fn get_intensity(values: &Matrix<Phasor>) -> Matrix<f64> {
	let new_values = values.raw().iter().map(|c| intensity(c)).collect();

	matrix::new(new_values, values.shape())
}

fn intensity(e: &Phasor) -> f64 {
    let (r, _theta) = e.clone().to_polar();
    
    // Intensidade é proporcional |e|²
    return r.powf(2.0);
}