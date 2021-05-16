pub mod list;
pub mod matrix;
pub mod comprehension;

pub type List<T> = Vec<T>;

#[derive(Clone)]
pub struct Matrix<T: Clone + Copy> {
    values: List<T>,
    shape: List<usize>
}

pub struct MatrixView<'a, T: Copy> {
    matrix: &'a Matrix<T>,
    shape_mask: List<usize>,
    position_mask: List<usize>
}

pub fn new_2d<T: Clone + Copy>(values: List<List<T>>, shape: &List<usize>) -> Matrix<T> {
    let raw_values = values.into_iter().flatten().collect::<List<T>>();

    matrix::new(raw_values, shape)
}

pub fn new_3d<T: Clone + Copy>(values: List<List<List<T>>>, shape: &List<usize>) -> Matrix<T> {
    let raw_values = values.into_iter().flatten().flatten().collect::<List<T>>();

    matrix::new(raw_values, shape)
}

pub fn head_or_default<T>(l: impl DoubleEndedIterator<Item = T>, default: T) -> T {
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

pub fn head<T>(mut l: impl DoubleEndedIterator<Item = T>) -> Option<T> {
	l.next()
}

pub fn last<T>(l: impl DoubleEndedIterator<Item = T>) -> Option<T> {
	l.last()
}

pub fn init<T: Clone>(l: impl DoubleEndedIterator<Item = T>) -> impl DoubleEndedIterator<Item = T> {
	let mut rev_iter = l.rev();
	rev_iter.next();
	rev_iter.rev()

}

pub fn tail<T: Clone>(mut l: impl DoubleEndedIterator<Item = T>) -> impl DoubleEndedIterator<Item = T> {
	l.next();
	l
}

pub fn middle<T: Clone>(l: impl DoubleEndedIterator<Item = T>) -> impl DoubleEndedIterator<Item = T> {
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