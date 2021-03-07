pub struct Array2d {
    pub dx: f64,
	pub dz: f64,
	
	pub xdelta: f64,
	pub zdelta: f64,

	pub xsteps: usize,
	pub zsteps: usize,
}

impl Array2d {
	pub fn new(dx: f64, xdelta: f64, dz: f64, zdelta: f64) -> Array2d {
    
		let xsteps = (dx / xdelta).round() as usize;
		let zsteps = (dz / zdelta).round() as usize;
		
		Array2d{
			dx: dx,
			dz: dz,
			xsteps: xsteps,
			zsteps: zsteps,
			xdelta: xdelta,
			zdelta: zdelta,
		}
	}

    pub fn get_x_points(&self) -> impl Iterator<Item=f64> + '_ {
		return (0usize..self.xsteps).map(move |x| (x as f64) * self.xdelta);
	}

    pub fn get_z_points(&self) -> impl Iterator<Item=f64> + '_ {
		return (0usize..self.zsteps).map(move |z| (z as f64) * self.zdelta);
	}
}