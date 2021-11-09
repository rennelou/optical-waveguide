use super::*;

#[derive(Clone)]
pub struct Rectilinear<const D: usize> {
    pub position: f64,
    n: f64,
    n0: f64,
    core_left: f64,
    core_right: f64
}

pub fn new_2d(n: f64, n0: f64, position: f64, core_width: f64) -> Rectilinear<2> {
    let core_left = position - (core_width/2.0);
    let core_right = position + (core_width/2.0);
    
    Rectilinear { position, n, n0, core_left, core_right }
}

pub fn new_3d(n: f64, n0: f64, position: f64, core_width: f64) -> Rectilinear<3> {
    let core_left = position - (core_width/2.0);
    let core_right = position + (core_width/2.0);
    
    Rectilinear { position, n, n0, core_left, core_right }
}

impl Core<2> for Rectilinear<2> {
    
    fn get_n(&self, grid: &Grid<2>, position: &[usize], n0: f64) -> f64 {
        let position = &position[1..];
        let deltas = &grid.get_deltas()[1..];
        
        let x = position[0] as f64 * deltas[0];
        if x > self.core_left && x < self.core_right {
            self.n
        } else {
            n0
        }   
    }

    fn get_n0(&self) -> f64 {
        return self.n0;
    }
}

impl Core<3> for Rectilinear<3> {
    
    fn get_n(&self, grid: &Grid<3>, position: &[usize], n0: f64) -> f64 {
        let position = &position[1..];
        let deltas = &grid.get_deltas()[1..];
        
        let y = position[0] as f64 * deltas[0];
        let x = position[1] as f64 * deltas[1];
        if y > self.core_left && y < self.core_right && x > self.core_left && x < self.core_right {
            self.n
        } else {
           n0
        }  
    }

    fn get_n0(&self) -> f64 {
        return self.n0;
    }
}
