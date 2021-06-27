pub mod rectilinear;
pub trait Core<const D: usize> {
    fn get_dimension(&self) -> usize {
        self.get_shape().len()
    }

    fn get_shape(&self) -> &[usize;D];

    fn get_deltas(&self) -> &[f64;D];

    fn get_n(&self, position: &[usize], n0: f64) -> f64;

    fn get_half_n(&self, position: &[usize], n0: f64) -> f64 { 
        let z = position[0];
        let mut position_z_foward = position.to_vec();
        position_z_foward[0] = z + 1;
        
        (self.get_n(position, n0) + self.get_n(position_z_foward.as_slice(), n0))/2.0
    }

    fn get_n0(&self) -> f64;
} 