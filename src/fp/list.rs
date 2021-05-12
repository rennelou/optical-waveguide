use std::ops::Add;

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

pub fn sum<T>(l1: &List<T>, l2: &List<T>) -> List<T> 
where T: Copy + Add<T, Output = T> {
    l1.iter().zip(l2.iter()).map(|(&x, &y)| x + y).collect()
}

pub fn drop_at<T: Clone>(l: &List<T>, i: usize) -> List<T> {
    let l_as_slice = l.as_slice();
    [&l_as_slice[0..i], &l_as_slice[i+1..l_as_slice.len()]].concat().to_vec()
}