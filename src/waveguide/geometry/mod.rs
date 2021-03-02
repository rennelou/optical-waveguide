use super::fp::list::List;

pub struct SlabGeometry {
    pub dx: f64,
	pub dz: f64,
	
	pub xdelta: f64,
	pub zdelta: f64,

	pub xsteps: usize,
	pub zsteps: usize,
}

pub fn new(dx: f64, xdelta: f64, dz: f64, zdelta: f64) -> SlabGeometry {
    
    let xsteps = (dx / xdelta).round() as usize;
    let zsteps = (dz / zdelta).round() as usize;
    
    SlabGeometry{
		dx: dx,
		dz: dz,
        xsteps: xsteps,
        zsteps: zsteps,
        xdelta: xdelta,
		zdelta: zdelta,
    }
}

impl SlabGeometry {
    pub fn get_x_points(&self) -> List<f64> {
		return (0usize..self.xsteps).map(|x| (x as f64) * self.xdelta).collect();
	}

    pub fn get_z_points(&self) -> List<f64> {
		return (0usize..self.zsteps).map(|z| (z as f64) * self.zdelta).collect();
	}
}