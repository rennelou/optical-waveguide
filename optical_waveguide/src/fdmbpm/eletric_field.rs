use crate::fp::Matrix;
use crate::fp::matrix;
use super::Phasor;

pub struct EletricField {
    values: Matrix<Phasor>,
    grid_steps: Vec<f64>
}

pub fn new(values: Matrix<Phasor>, grid_steps: Vec<f64>) -> EletricField {
    EletricField { values, grid_steps }
}

impl EletricField {
    pub fn get_eletric_fields(&self) -> Matrix<f64>  {
        let values = self.values.raw().iter().map(|p|{
            let (r, _theta) = p.clone().to_polar();

            r
        }).collect();

        matrix::new_from_raw(values, self.shape())
    }

    pub fn get_intensity(&self) -> Matrix<f64> {
        let values = self.values.raw().iter().map(|c| intensity(c)).collect();

        matrix::new_from_raw(values, self.shape())
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