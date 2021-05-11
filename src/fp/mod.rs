pub mod list;
//pub mod matrix;
pub mod comprehension;

pub type List<T> = Vec<T>;

pub struct Matrix<T: Clone, const N: usize> {
    values: List<T>,
    shape: [usize;N]
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