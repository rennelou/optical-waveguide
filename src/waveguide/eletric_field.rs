use super::Phasor;
use super::EletricField;
use crate::fp::List;

impl EletricField {
    pub fn get_intensity(&self) -> List<f64> {
        self.values.raw().iter().map(|c| intensity(c)).collect()
    }

    pub fn shape(&self) -> &List<usize> {
        self.values.shape()
    }

    pub fn grid_steps(&self) -> &List<f64> {
        &self.grid_steps
    }
}

fn intensity(e: &Phasor) -> f64 {
    let (r, _theta) = e.clone().to_polar();
    
    // Intensidade é proporcional |e|²
    return r.abs().powf(2.0);
}