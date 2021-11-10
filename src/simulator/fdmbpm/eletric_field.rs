use crate::tools::export;
use crate::functional_types::{Matrix, matrix};
use super::*;
use types::*;

pub struct EletricField {
    values: Matrix<Phasor>,
    grid_steps: Vec<f64>,
    refractive_indexes: Vec<f64>
}

pub fn new(values: Matrix<Phasor>, grid_steps: Vec<f64>, refractive_indexes: Vec<f64>) -> EletricField {
    EletricField { values, grid_steps, refractive_indexes }
}

impl SimulationResults for EletricField{
    fn export(&self, output_name: &str) {
        
        let shape = self.shape();
        let intensity = self.get_intensity();
        let eletric_field = self.get_eletric_fields();
        let deltas = self.grid_steps();
        
        let core_matrix = self.refractive_indexes.clone();

        export::hdf5(
            output_name,
            shape,
            deltas,
            eletric_field,
            intensity,
            core_matrix
        );
    }
}

impl EletricField {
    pub fn get_eletric_fields(&self) -> Matrix<f64>  {
        let values = self.values.raw().iter().map(|p|{
            let (r, _theta) = p.clone().to_polar();

            r
        }).collect();

        matrix::new(values, self.shape())
    }

    pub fn get_intensity(&self) -> Matrix<f64> {
        let values = self.values.raw().iter().map(|c| intensity(c)).collect();

        matrix::new(values, self.shape())
    }

    pub fn shape(&self) -> &[usize] {
        self.values.shape()
    }

    pub fn grid_steps(&self) -> &[f64] {
        &self.grid_steps
    }
}

fn intensity(e: &Phasor) -> f64 {
    let (r, _theta) = e.clone().to_polar();
    
    // Intensidade é proporcional |e|²
    return r.powf(2.0);
}