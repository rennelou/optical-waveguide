use super::Phasor;
use super::EletricField;
use super::Intensity;
use crate::fp::list;

#[derive(Debug, Clone, Copy)]
pub struct Point2d{
    pub z: f64, 
    pub x: f64,
    pub intensity: f64,
}

impl EletricField {
    pub fn get_points(&self) -> impl Iterator<Item=impl Iterator<Item=Point2d> + '_> + '_ {
        let (zdelta, xdelta) = self.deltas;
        let (zsteps, xsteps) = self.shape;

        let zpoints = (0usize..zsteps).map(
            move |z| (z as f64) * zdelta
        );
        
        return zpoints.zip(&self.es).map(move |(z, l)| {
            
            let xpoints = (0usize..xsteps).map(
                move |x| (x as f64) * xdelta
            );
            
            return xpoints.zip(l).map(move |(x, c)| {
                let intensity = intensity(c);
                
                Point2d{ z, x, intensity }
            });    
        });
    }

    pub fn get_intensity(&self) -> Intensity {
        let dimensions = self.shape;
        let deltas = self.deltas;

        let values = self.es.iter().fold(
            list::empty(), 
            |acc, l| acc.into_iter().chain(
                    l.iter().map(|c| intensity(c))
                ).collect()
        );

        Intensity { shape: dimensions, deltas, values }
    }
}

fn intensity(e: &Phasor) -> f64 {
    let (r, _theta) = e.clone().to_polar();
    
    // Intensidade é proporcional |e|²
    return r.abs().powf(2.0);
}