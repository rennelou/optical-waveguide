use crate::fp;
use super::*;

pub fn new<T: Clone + Copy>(values: Vec<T>) -> Matrix<T> {
    let shape = vec![values.len()];
    
    Matrix { values, shape }
}

pub fn new_from_raw<T: Clone + Copy>(values: Vec<T>, shape_ref: &[usize]) -> Matrix<T> {
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
            list::concat(result, m.into_raw())
        }
    );

    new_from_raw(new_values, &new_shape)
}

impl<T: Clone + Copy> Matrix<T> {
    pub fn raw(&self) -> &Vec<T> {
        &self.values
    }

    pub fn into_raw(self) -> Vec<T> {
        self.values
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn get(&self, position: &[usize]) -> &T {
        &self.values[Matrix::<T>::position_to_id(position, self.shape())]
    }

    fn position_to_id(position: &[usize], shape: &[usize]) -> usize {
        (0..position.len()).fold(0, |id, index| {
            id*shape[index]+position[index]
        })
    }
}

// #Todo otimizar essa função
pub fn dephts_cartesian_product(shape: Vec<usize>) -> Vec<Vec<usize>> {
    (0..shape.iter().product()).map(
        |id| matrix::id_to_position(id, shape.as_slice())
    ).collect()
}

fn id_to_position(id: usize, shape: &[usize]) -> Vec<usize> {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acess_position() {
        let matrix = new_from_raw(vec![0,1,2,3,4,5], &[2usize, 3usize]);

        assert_eq!(matrix.get(&[0,0]), &0);
        assert_eq!(matrix.get(&[0,1]), &1);
        assert_eq!(matrix.get(&[0,2]), &2);
        assert_eq!(matrix.get(&[1,0]), &3);
        assert_eq!(matrix.get(&[1,1]), &4);
        assert_eq!(matrix.get(&[1,2]), &5);
    }

    #[test]
    fn zip_test() {
        let m1 = new_from_raw(vec![0,1,2,3,4,5], &[2usize, 3usize]);
        let m2 = new_from_raw(vec![6,7,8,9,10,11], &[2usize, 3usize]);

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