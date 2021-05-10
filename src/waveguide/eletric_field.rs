use super::Phasor;
use super::EletricField;
use super::Intensity;
use crate::fp::list;

impl<const N: usize> EletricField<N> {
    pub fn get_intensity(&self) -> Intensity<N> {
        let shape = self.shape;
        let deltas = self.deltas;

        let values = self.es.iter().fold(
            list::empty(), 
            |acc, l| acc.into_iter().chain(
                    l.iter().map(|c| intensity(c))
                ).collect()
        );

        Intensity { shape, deltas, values }
    }
}

fn intensity(e: &Phasor) -> f64 {
    let (r, _theta) = e.clone().to_polar();
    
    // Intensidade é proporcional |e|²
    return r.abs().powf(2.0);
}