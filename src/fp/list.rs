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

pub fn append<T>(mut list: List<T>, value: T) -> List<T> {
    list.push(value);
    
    return list;
}

pub fn concat<T>(mut init: List<T>, mut tail: List<T>) -> List<T> {
    init.append(&mut tail);
    
    return init;
}

// ------------- Funções abaixo estão preparadas para receber qualuqer iterator ---------------
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
	
	l.iter().next().cloned()
}

pub fn last<T: Clone>(l: &List<T>) -> Option<T> {
	if l.is_empty() {
		return None;
	}
	
	l.iter().last().cloned()
}

pub fn init<T: Clone>(l: &List<T>) -> List<T> {
	if l.is_empty() {
		return vec![];
	}

	//let mut result: List<T> = vec![];
	//result.extend_from_slice(&l[0..l.len()-1]);
	//return result;
	
	// Um pouco mais lento que o codigo comentado acima, porem funciona pra qualquer iterator
	let mut rev_iter = l.iter().rev();
	rev_iter.next();
	rev_iter.rev().cloned().collect()

}

pub fn tail<T: Clone>(l: &List<T>) -> List<T> {
	if l.is_empty() {
		return vec![];
	}

	//let mut result: List<T> = vec![];
	//result.extend_from_slice(&l[1..]);
	//return result;
	
	// Um pouco mais lento que o codigo comentado acima, porem funciona pra qualquer iterator
	let mut iter = l.iter();
	iter.next();
	iter.cloned().collect()
}

pub fn body<T: Clone>(l: &List<T>) -> List<T> {
	if l.is_empty() {
		return vec![];
	}

	return init(&tail(&l));
}