use super::List;

pub fn new<T>(v: T) -> List<T> {
    return append(empty(), v);
}

pub fn empty<T>() -> List<T> {
    return vec![];
}

pub fn append<T>(mut list: List<T>, value: T) -> List<T> {
    list.push(value);
    
    return list;
}

pub fn concat<T>(mut init: List<T>, mut tail: List<T>) -> List<T> {
    init.append(&mut tail);
    
    return init;
}