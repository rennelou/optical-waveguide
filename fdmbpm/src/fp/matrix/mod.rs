use crate::fp;
use super::*;

#[derive(Clone, Copy)]
pub enum Idx {
    Free,
    Value(usize)
}

pub fn new<T: Clone + Copy>(values: Vec<T>, shape_ref: &[usize]) -> Matrix<T> {
    if shape_ref.iter().product::<usize>() != values.len() {
        panic!("shape dosent match with values")
    }
    let shape = shape_ref.to_vec();
    
    Matrix { values, shape }
}

pub fn new2_from_vec_vec<T: Clone + Copy>(values: Vec<Vec<T>>) -> Matrix<T> {
    let y_depht = values.len();
    let x_depht = fp::head(values.iter()).unwrap().len();

    if values.iter().any(|v| v.len() != x_depht) {
        panic!("all lines needs have the same lenght")
    }
    
    let new_values = values.into_iter().flatten().collect();

    Matrix { values: new_values, shape: vec![y_depht, x_depht] }
}

pub fn new_from_vec<T: Clone + Copy>(matrixs: Vec<Matrix<T>>) -> Matrix<T> {
    if (0..matrixs.len()-1).any(|i| matrixs[i].shape() != matrixs[i+1].shape()) {
        panic!("all matrixs must have the sames shapes")
    }

    let shape = fp::head(matrixs.iter()).unwrap().shape();

    let new_depht = matrixs.len();
    let mut new_shape = shape.to_vec();
    new_shape.insert(0, new_depht);

    let new_values = matrixs.into_iter().fold(
        vec![],
        |result, m| {
            list::concat(result, m.taken_raw())
        }
    );

    new(new_values, &new_shape)
}

impl<T: Clone + Copy> Matrix<T> {
    pub fn raw(&self) -> &Vec<T> {
        &self.values
    }

    fn taken_raw(self) -> Vec<T> {
        self.values
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn get(&self, position: &[usize]) -> &T {
        &self.values[position_to_id(position, self.shape())]
    }

    pub fn get_transposed(&self, position: &[usize]) -> &T {
        let reversed_position: Vec<_> = position.iter().rev().copied().collect();
        let id = position_to_id(reversed_position.as_slice(), self.shape());
        &self.values[id]
    }
}

pub fn id_to_position(id: usize, shape: &[usize]) -> Vec<usize> {
    let len = shape.len();
    
    let mut position = vec![0usize;len];
    let mut quocient = id;
    for i in 1..=len {
        let rev_i = len - i;
        let depht = shape[rev_i];
        position[rev_i] = quocient%depht;
        quocient = quocient/depht;
    }

    position
}

pub fn position_to_id(position: &[usize], shape: &[usize]) -> usize {
    (0..position.len()).fold(0, |id, index| {
        id*shape[index]+position[index]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acess_position() {
        let matrix = new(vec![0,1,2,3,4,5], &[2usize, 3usize]);

        assert_eq!(matrix.get(&[0,0]), &0);
        assert_eq!(matrix.get(&[0,1]), &1);
        assert_eq!(matrix.get(&[0,2]), &2);
        assert_eq!(matrix.get(&[1,0]), &3);
        assert_eq!(matrix.get(&[1,1]), &4);
        assert_eq!(matrix.get(&[1,2]), &5);
    }

    #[test]
    fn transposed_test() {
        let matrix = new(vec![0,1,2,3,4,5], &[2usize, 3usize]);

        assert_eq!(matrix.get_transposed(&[0,0]), &0);
        assert_eq!(matrix.get_transposed(&[0,1]), &3);
        assert_eq!(matrix.get_transposed(&[1,0]), &1);
        assert_eq!(matrix.get_transposed(&[1,1]), &4);
        assert_eq!(matrix.get_transposed(&[2,0]), &2);
        assert_eq!(matrix.get_transposed(&[2,1]), &5);
    }

    #[test]
    fn zip_test() {
        let m1 = new(vec![0,1,2,3,4,5], &[2usize, 3usize]);
        let m2 = new(vec![6,7,8,9,10,11], &[2usize, 3usize]);

        let ziped = matrix::new_from_vec(vec![m1, m2]);
        
        assert_eq!(ziped.get(&[0,0,0]), &0);
        assert_eq!(ziped.get(&[0,0,1]), &1);
        assert_eq!(ziped.get(&[0,0,2]), &2);
        assert_eq!(ziped.get(&[0,1,0]), &3);
        assert_eq!(ziped.get(&[0,1,1]), &4);
        assert_eq!(ziped.get(&[0,1,2]), &5);
        assert_eq!(ziped.get(&[1,0,0]), &6);
        assert_eq!(ziped.get(&[1,0,1]), &7);
        assert_eq!(ziped.get(&[1,0,2]), &8);
        assert_eq!(ziped.get(&[1,1,0]), &9);
        assert_eq!(ziped.get(&[1,1,1]), &10);
        assert_eq!(ziped.get(&[1,1,2]), &11);
    }
}