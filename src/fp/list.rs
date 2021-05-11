use super::List;

pub fn new<T>(v: T) -> List<T> {
    return append(empty(), v);
}

pub fn empty<T>() -> List<T> {
    return vec![];
}

pub fn new_from_slice<T: Clone>(s: &[T]) -> List<T> {
	let mut result: Vec<T> = vec![];
	result.extend_from_slice(s);

	return result;
}

pub fn head_or_default<T: Clone>(list: &List<T>, default: T) -> T {
	return super::unwrap_or_default(
		head(&list), 
		default
	);
}

pub fn last_or_default<T: Clone>(list: &List<T>, default: T) -> T {
	return super::unwrap_or_default(
		last(&list), 
		default
	);
}

pub fn head<T: Clone>(l: &List<T>) -> Option<T> {
	if l.is_empty() {
		return None
	}
	let first_index = 0usize;
	return Some(l[first_index].clone());
}

pub fn last<T: Clone>(l: &List<T>) -> Option<T> {
	if l.is_empty() {
		return None;
	}
	let last_index = (l.len()-1) as usize;
	return Some(l[last_index].clone());
}

pub fn init<T: Clone>(l: &List<T>) -> List<T> {
	if l.is_empty() {
		return vec![];
	}

	let mut result: List<T> = vec![];
	result.extend_from_slice(&l[0..l.len()-1]);
	return result;
}

pub fn tail<T: Clone>(l: &List<T>) -> List<T> {
	if l.is_empty() {
		return vec![];
	}

	let mut result: List<T> = vec![];
	result.extend_from_slice(&l[1..]);
	return result;
}

pub fn body<T: Clone>(l: &List<T>) -> List<T> {
	if l.is_empty() {
		return vec![];
	}

	return init(&tail(&l));
}

pub fn append<T>(mut list: List<T>, value: T) -> List<T> {
    list.push(value);
    
    return list;
}

pub fn concat<T>(mut init: List<T>, mut tail: List<T>) -> List<T> {
    init.append(&mut tail);
    
    return init;
}