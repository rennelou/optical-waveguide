use num::Complex;
use super::slab::Slab2d;
use crate::fp::list::List;

pub struct EletricField2d {
    pub es: List<List<Complex<f64>>>,

    pub xdelta: f64,
    pub zdelta: f64,

    pub xsteps: usize,
    pub zsteps: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Point2d{
    pub z: f64, 
    pub x: f64,
    pub eletric_field: f64,
}

pub fn new(w: &Slab2d, es: List<List<Complex<f64>>>) -> EletricField2d {
    return EletricField2d {
        es: es,
        zdelta: w.zdelta,
        xdelta: w.xdelta,
        zsteps: w.zsteps,
        xsteps: w.xsteps,
    };
}

impl EletricField2d {
    pub fn get_points(&self) -> impl Iterator<Item=impl Iterator<Item=Point2d> + '_> + '_ {
        let zpoints = (0usize..self.zsteps).map(move |z| (z as f64) * self.zdelta);
        
        return zpoints.zip(&self.es).map(move |(z, l)| {
            
            let xpoints = (0usize..self.xsteps).map(move |x| (x as f64) * self.xdelta);
            
            return xpoints.zip(l).map(move |(x, c)| {
                
                let (r, _theta) = c.clone().to_polar();
                // Intensidade é proporcional |e|²
                let eletric_field = r.abs().powf(2.0);
                
                Point2d{
                    z, 
                    x, 
                    eletric_field
                }
            });    
        });
    }
}