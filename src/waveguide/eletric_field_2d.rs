use num::Complex;
use super::slab::Slab;
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

pub fn new(w: &Slab, es: List<List<Complex<f64>>>) -> EletricField2d {
    return EletricField2d {
        es: es,
        zdelta: w.zdelta,
        xdelta: w.xdelta,
        zsteps: w.zsteps,
        xsteps: w.xsteps,
    };
}

impl EletricField2d {
    pub fn get_points(&self) -> List<List<Point2d>> {
        let zpoints: List<f64> = (0usize..self.zsteps).map(|x| (x as f64) * self.zdelta).collect();
        let xpoints: List<f64> = (0usize..self.xsteps).map(|x| (x as f64) * self.xdelta).collect();
        
        return zpoints.iter().copied().zip(&self.es).map(|(z, l)| {
                let es_real = l.iter().map(|c| {
                    let (r, theta) = c.clone().to_polar();
                    r * theta.cos()
                });

                let x_and_es: List<(f64, f64)> = xpoints.iter().copied().zip(es_real).collect();

                return x_and_es.iter().copied().map(|(x, eletric_field)|
                    Point2d{
                        z, 
                        x, 
                        eletric_field
                    }
                ).collect();
            }
        ).collect();
    }
}