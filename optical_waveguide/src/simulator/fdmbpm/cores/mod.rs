use super::grid::Grid;

pub mod rectilinear;

pub enum AlTypeCore {
    Bidimensional(Box<dyn Core<2>>),
    Tridimensional(Box<dyn Core<3>>)
}

pub trait Core<const D: usize> {
    fn get_n(&self, grid: &Grid<D>, position: &[usize], n0: f64) -> f64;

    fn get_half_n(&self, grid: &Grid<D>, position: &[usize], n0: f64) -> f64 { 
        let z = position[0];
        
        let mut position_z_foward = [0usize;D];
        position_z_foward[0] = z + 1;
        
        for i in 1..D {
            position_z_foward[i] = position[i];
        }
        
        (self.get_n(grid, position, n0) + self.get_n(grid, &position_z_foward, n0))/2.0
    }

    fn get_n0(&self) -> f64;
}