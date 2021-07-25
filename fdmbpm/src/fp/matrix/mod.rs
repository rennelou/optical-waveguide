use crate::fp;
use super::*;

#[derive(Clone, Copy)]
pub enum Idx {
    Free,
    Value(usize)
}

pub fn new<T: Clone + Copy, const D: usize>(values: Vec<T>, shape_ref: &[usize;D]) -> Matrix<T,D> {
    if shape_ref.iter().product::<usize>() != values.len() {
        panic!("shape dosent match with values")
    }
    let shape = shape_ref.clone();
    
    Matrix { values, shape }
}

pub fn new_from_vec<T, const D: usize, const N: usize>(matrixs: Vec<Matrix<T, D>>) -> Matrix<T, N> 
where T: Copy {
    if N != D + 1 {
        panic!("so pode subir uma dimensão e eu to puto que to brigando com a linguagem pra isso ser garantido em tempo de compilaçao")
    }  

    let all_shapes_equals = (0..matrixs.len()-1).any(
        |i| matrixs[i].shape() != matrixs[i+1].shape()
    );
    
    if all_shapes_equals {
        panic!("all matrixs must have the sames shapes")
    }
    
    let shape = fp::head(matrixs.iter()).unwrap().shape();

    let new_depht = matrixs.len();
    let mut new_shape = [0;N];
    
    new_shape[0] = new_depht;
    let mut index = 1usize;
    for &depht in shape {
        new_shape[index] = depht;
        index = index + 1;
    }

    let new_values = matrixs.into_iter().fold(
        vec![],
        |result, m| {
            list::concat(result, m.taken_raw())
        }
    );

    new(new_values, &new_shape)
}

impl<T: Clone + Copy, const D: usize> Matrix<T, D> {
    pub fn raw(&self) -> &Vec<T> {
        &self.values
    }

    fn taken_raw(self) -> Vec<T> {
        self.values
    }

    pub fn shape(&self) -> &[usize;D] {
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

        let ziped = matrix::new_from_vec::<i32,2,3>(vec![m1, m2]);
        
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