use std::ops::Add;

pub fn append<T>(mut list: Vec<T>, value: T) -> Vec<T> {
    list.push(value);
    
    return list;
}

pub fn concat<T>(mut init: Vec<T>, mut tail: Vec<T>) -> Vec<T> {
    init.append(&mut tail);
    
    return init;
}

pub fn sum<T>(l1: &Vec<T>, l2: &Vec<T>) -> Vec<T> 
where T: Copy + Add<T, Output = T> {
    l1.iter().zip(l2.iter()).map(|(&x, &y)| x + y).collect()
}

pub fn drop_at<T: Clone>(l: &Vec<T>, i: usize) -> Vec<T> {
    let l_as_slice = l.as_slice();
    [&l_as_slice[0..i], &l_as_slice[i+1..l_as_slice.len()]].concat().to_vec()
}