use super::fp::list;

#[derive(Clone, Copy)]
pub struct Grid {
	pub d: f64,
	pub delta: f64,
	pub steps: usize
}

impl Grid {

	pub fn new(d: f64, delta: f64) -> Grid {
		let steps = (d/delta).round() as usize;

		Grid {
			d, 
			delta,
			steps
		}
	}

	pub fn get_points(&self) -> impl Iterator<Item=f64> + '_ {
		return (0usize..self.steps).map(move |i| (i as f64)*self.delta);
	}

	pub fn get_indexs(&self) -> impl Iterator<Item=f64> + '_ {
		return (0usize..self.steps).map(move |i| i as f64);
	}
}

#[derive(Clone)]
pub struct Grid2d {
    values: list::List<Grid>
}

impl Grid2d {
	const X: usize = 0;
	const Z: usize = 1;

	pub fn new(dx: f64, xdelta: f64, dz: f64, zdelta: f64) -> Grid2d {
    
		let array0 = Grid::new(dx,xdelta);
		let array1 = Grid::new(dz,zdelta);
		let values = list::append(list::new(array0),array1);

		Grid2d {values}
	}

	pub fn get_x(&self) -> &Grid {
		return &self.values[Grid2d::X];
	}

	pub fn get_z(&self) -> &Grid {
		return &self.values[Grid2d::Z];
	}
}