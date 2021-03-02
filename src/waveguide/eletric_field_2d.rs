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
    pub fn get_points(&self) -> List<List<Point2d>> {
        let zpoints = (0usize..self.zsteps).map(|x| (x as f64) * self.zdelta);
        
        return zpoints.zip(&self.es).map(|(z, l)| {
                
                let xpoints = (0usize..self.xsteps).map(|x| (x as f64) * self.xdelta);    

                let es_real = l.iter().map(|c| {
                    let (r, theta) = c.clone().to_polar();
                    r * theta.cos()
                });

                return xpoints.zip(es_real).map(|(x, eletric_field)|
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