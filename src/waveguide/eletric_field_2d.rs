use num::Complex;
use super::slab::Slab2d;
use crate::fp::list::List;
use crate::array::Array2d;

const X: usize = 0;
const Z: usize = 1;

pub struct EletricField2d {
    pub es: List<List<Complex<f64>>>,

    pub grid: Array2d
}

#[derive(Debug, Clone, Copy)]
pub struct Point2d{
    pub z: f64, 
    pub x: f64,
    pub eletric_field: f64,
}

pub fn new(w: &Slab2d, es: List<List<Complex<f64>>>) -> EletricField2d {
    let grid = &w.grid;

    return EletricField2d { es, grid: grid.clone() };
}

impl EletricField2d {
    pub fn get_points(&self) -> impl Iterator<Item=impl Iterator<Item=Point2d> + '_> + '_ {
        let xdelta = self.grid.get(X).delta;
        let xsteps = self.grid.get(X).steps;
        let zdelta = self.grid.get(Z).delta;
        let zsteps = self.grid.get(Z).steps;

        let zpoints = (0usize..zsteps).map(move |z| (z as f64) * zdelta);
        
        return zpoints.zip(&self.es).map(move |(z, l)| {
            
            let xpoints = (0usize..xsteps).map(move |x| (x as f64) * xdelta);
            
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