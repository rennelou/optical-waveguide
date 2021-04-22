use super::fp::list;

pub struct Array {
	pub d: f64,
	pub delta: f64,
	pub steps: usize
}

impl Array {
	const EMPTY: Array = Array{d:0.0,delta:0.0,steps:0};

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

pub struct Array2d {
    values: list::List<Array>
}

impl Array2d {
	const DIMENSION: usize = 2;

	pub fn new(dx: f64, xdelta: f64, dz: f64, zdelta: f64) -> Array2d {
    
		let array0 = Array::new(dx,xdelta);
		let array1 = Array::new(dz,zdelta);
		let values = list::append(list::new(array0),array1);

		Array2d {values}
	}

	pub fn get(&self, index: usize) -> &Array {
		if index < Array2d::DIMENSION {
			return &self.values[index];
		}

		return &Array::EMPTY;
	}
}