use super::fp::list;

#[derive(Clone, Copy)]
pub struct Array {
	pub d: f64,
	pub delta: f64,
	pub steps: usize
}

impl Array {

	pub fn new(d: f64, delta: f64) -> Array {
		let steps = (d/delta).round() as usize;

		Array {
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
pub struct Array2d {
    values: list::List<Array>
}

impl Array2d {
	const X: usize = 0;
	const Z: usize = 1;

	pub fn new(dx: f64, xdelta: f64, dz: f64, zdelta: f64) -> Array2d {
    
		let array0 = Array::new(dx,xdelta);
		let array1 = Array::new(dz,zdelta);
		let values = list::append(list::new(array0),array1);

		Array2d {values}
	}

	pub fn get_x(&self) -> &Array {
		return &self.values[Array2d::X];
	}

	pub fn get_z(&self) -> &Array {
		return &self.values[Array2d::Z];
	}
}