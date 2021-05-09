use num::Complex;
use super::slab::Slab2d;
use crate::fp::list;
use crate::fp::list::List;

pub struct EletricField2d {
    pub es: List<List<Complex<f64>>>,
    pub dimensions: (usize, usize),
    pub deltas: (f64, f64)
}

pub struct Intensity {
    pub values: List<f64>,
    pub dimensions: (usize, usize),
    pub deltas: (f64, f64)
}

#[derive(Debug, Clone, Copy)]
pub struct Point2d{
    pub z: f64, 
    pub x: f64,
    pub intensity: f64,
}

pub fn new(w: &Slab2d, es: List<List<Complex<f64>>>) -> EletricField2d {
    let grid = &w.grid;
    let xdelta = grid.get_x().delta;
    let xsteps = grid.get_x().steps;
    let zdelta = grid.get_z().delta;
    let zsteps = grid.get_z().steps;

    let dimensions = (xsteps, zsteps);
    let deltas = (xdelta, zdelta);

    return EletricField2d { es, dimensions, deltas };
}

impl EletricField2d {
    pub fn get_points(&self) -> impl Iterator<Item=impl Iterator<Item=Point2d> + '_> + '_ {
        let (xdelta, zdelta) = self.deltas;
        let (xsteps, zsteps) = self.dimensions;

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
        let dimensions = self.dimensions;
        let deltas = self.deltas;

        let values = self.es.iter().fold(
            list::empty(), 
            |acc, l| acc.into_iter().chain(
                    l.iter().map(|c| intensity(c))
                ).collect()
        );

        Intensity { dimensions, deltas, values }
    }
}

fn intensity(e: &Complex<f64>) -> f64 {
    let (r, _theta) = e.clone().to_polar();
    
    // Intensidade é proporcional |e|²
    return r.abs().powf(2.0);
}