use crate::export;
use crate::fp::Matrix;
use crate::fp::matrix;
use super::Phasor;
use super::cores::AlTypeCore;
use super::cores::Core;
use super::grid::AlTypeGrid;
use super::grid::Grid;

pub struct EletricField {
    values: Matrix<Phasor>,
    grid_steps: Vec<f64>,
    grid: AlTypeGrid,
    core: AlTypeCore
}

pub fn new(values: Matrix<Phasor>, grid_steps: Vec<f64>, grid: AlTypeGrid, core: AlTypeCore) -> EletricField {
    EletricField { values, grid_steps, grid, core }
}

impl EletricField {
    pub fn get_eletric_fields(&self) -> Matrix<f64>  {
        let values = self.values.raw().iter().map(|p|{
            let (r, _theta) = p.clone().to_polar();

            r
        }).collect();

        matrix::new(values, self.shape())
    }

    pub fn get_intensity(&self) -> Matrix<f64> {
        let values = self.values.raw().iter().map(|c| intensity(c)).collect();

        matrix::new(values, self.shape())
    }

    pub fn shape(&self) -> &[usize] {
        self.values.shape()
    }

    pub fn grid_steps(&self) -> &[f64] {
        &self.grid_steps
    }

    pub fn export(self, output_name: &str) {
        
        let shape = self.shape();
        let intensity = self.get_intensity();
        let eletric_field = self.get_eletric_fields();
        let deltas = self.grid_steps();

        let core_matrix = match (&self.grid, &self.core) {
            (AlTypeGrid::Bidimensional(grid), AlTypeCore::Bidimensional(core)) => {
                get_core_matrix(grid, core)
            },
            (AlTypeGrid::Tridimensional(grid), AlTypeCore::Tridimensional(core)) => {
                get_core_matrix(grid, core)
            },
            _ => {
                panic!("Grid and Core mudt have the same dimension")
            }
        };

        export::hdf5(
            output_name,
            shape,
            deltas,
            eletric_field,
            intensity,
            core_matrix
        );
    }
}

fn get_core_matrix<const D: usize>(grid: &Grid<D>, core: &Box<dyn Core<D>>) -> Vec<f64> {
    let shape = grid.get_shape().to_vec();
    
    matrix::cartesian_product_of_shape(shape).map(
        |position| core.get_n(&grid, position.as_slice(), core.get_n0())
    ).collect()
}

fn intensity(e: &Phasor) -> f64 {
    let (r, _theta) = e.clone().to_polar();
    
    // Intensidade é proporcional |e|²
    return r.powf(2.0);
}