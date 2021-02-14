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
