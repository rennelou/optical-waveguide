use super::Phasor;
use super::EletricField;

impl EletricField {
    pub fn get_values(&self) -> Vec<f64> {
        self.values.raw().iter().map(|p|{
            let (r, _theta) = p.clone().to_polar();

            r
        }).collect()
    }

    pub fn get_intensity(&self) -> Vec<f64> {
        self.values.raw().iter().map(|c| intensity(c)).collect()
    }

    pub fn shape(&self) -> &Vec<usize> {
        self.values.shape()
    }

    pub fn grid_steps(&self) -> &Vec<f64> {
        &self.grid_steps
    }
}

fn intensity(e: &Phasor) -> f64 {
    let (r, _theta) = e.clone().to_polar();
    
    // Intensidade é proporcional |e|²
    return r.powf(2.0);
}