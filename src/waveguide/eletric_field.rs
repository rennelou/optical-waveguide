use super::Phasor;
use super::EletricField;
use crate::fp::List;

impl EletricField {
    pub fn get_intensity(&self) -> List<f64> {
        self.values.iter().map(|c| intensity(c)).collect()
    }
}

fn intensity(e: &Phasor) -> f64 {
    let (r, _theta) = e.clone().to_polar();
    
    // Intensidade é proporcional |e|²
    return r.abs().powf(2.0);
}