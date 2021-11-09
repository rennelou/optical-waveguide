pub fn append<T>(mut list: Vec<T>, value: T) -> Vec<T> {
    list.push(value);
    
    return list;
}

pub fn concat<T>(mut init: Vec<T>, mut tail: Vec<T>) -> Vec<T> {
    init.append(&mut tail);
    
    return init;
}