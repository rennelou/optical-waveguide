pub mod list;
pub mod matrix;
pub mod comprehension;

#[derive(Clone)]
pub struct Matrix<T: Clone + Copy> {
    values: Vec<T>,
    shape: Vec<usize>
}

pub fn new_2d<T: Clone + Copy>(values: Vec<Vec<T>>, shape: &Vec<usize>) -> Matrix<T> {
    let raw_values = values.into_iter().flatten().collect::<Vec<T>>();

    matrix::new(raw_values, shape)
}

pub fn new_3d<T: Clone + Copy>(values: Vec<Vec<Vec<T>>>, shape: &Vec<usize>) -> Matrix<T> {
    let raw_values = values.into_iter().flatten().flatten().collect::<Vec<T>>();

    matrix::new(raw_values, shape)
}

pub fn head_or_default<T>(l: impl Iterator<Item = T>, default: T) -> T {
	return unwrap_or_default(
		head(l), 
		default
	);
}

pub fn last_or_default<T>(l: impl DoubleEndedIterator<Item = T>, default: T) -> T {
	return unwrap_or_default(
		last(l), 
		default
	);
}

pub fn head<T>(mut l: impl Iterator<Item = T>) -> Option<T> {
	l.next()
}

pub fn last<T>(mut l: impl DoubleEndedIterator<Item = T>) -> Option<T> {
	l.next_back()
}

pub fn init<T: Clone>(mut l: impl DoubleEndedIterator<Item = T>) -> impl Iterator<Item = T> {
	l.next_back();
	l
}

pub fn tail<T: Clone>(mut l: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
	l.next();
	l
}

pub fn middle<T: Clone>(l: impl DoubleEndedIterator<Item = T>) -> impl Iterator<Item = T> {
	return tail(init(l));
}

pub fn unwrap_or_default<T>(wrap: Option<T>, default: T) -> T {
	return {
		if let None = wrap {
			default
		} else {
			wrap.unwrap()
		}
	};
}