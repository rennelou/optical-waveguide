use num::complex::Complex;

pub fn head<T: Copy>(l: &Vec<T>) -> Option<T> {
	if l.is_empty() {
		return None
	}
	let first_index = 0usize;
	return Some(l[first_index]);
}

pub fn last<T: Copy>(l: &Vec<T>) -> Option<T> {
	if l.is_empty() {
		return None;
	}
	let last_index = (l.len()-1) as usize;
	return Some(l[last_index]);
}

pub fn init<T: Copy>(l: &Vec<T>) -> Vec<T> {
	if l.is_empty() {
		return vec![];
	}

	let mut result: Vec<T> = vec![];
	result.extend_from_slice(&l[0..l.len()-1]);
	return result;
}

pub fn tail<T: Copy>(l: &Vec<T>) -> Vec<T> {
	if l.is_empty() {
		return vec![];
	}

	let mut result: Vec<T> = vec![];
	result.extend_from_slice(&l[1..]);
	return result;
}

pub fn multiply_complex(l: &Vec<Complex<f64>>, c: Complex<f64>) -> Vec<Complex<f64>> {
	return l.iter().map(|x| x * c).collect();
}
