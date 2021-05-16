use crate::fp;
use super::*;

pub mod matrix_view;

#[derive(Clone, Copy)]
pub enum Index {
    Free,
    Value(usize)
}

pub fn new<T: Clone + Copy>(values: Vec<T>, shape_ref: &Vec<usize>) -> Matrix<T> {
    if shape_ref.iter().product::<usize>() != values.len() {
        panic!("shape dosent match with values")
    }
    let shape = shape_ref.clone();
    
    Matrix { values, shape }
}

impl<T: Clone + Copy> Matrix<T> {
    pub fn raw(&self) -> &Vec<T> {
        &self.values
    }

    fn taken_raw(self) -> Vec<T> {
        self.values
    }

    pub fn shape(&self) -> &Vec<usize> {
        &self.shape
    }

    pub fn dimension(&self) -> usize {
        self.shape.iter().copied().fold(
            0, 
            |dim, depht|{ if depht > 1 {dim + 1} else {dim} }
        )
    }

    pub fn is_dimensionless(&self) -> bool {
        self.values.len() == 1
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn to_vec(&self) -> Vec<T> {
        if self.dimension() != 1 {
            panic!("matrix must have be unidimensional to be converted in Vec")
        }
    
        self.raw().clone()
    }

    pub fn get(&self, position: Vec<usize>) -> T {
        self.values[hash(position, &self.shape)]
    }

    pub fn view(&self, slice: &[Index]) -> MatrixView<T> {
        if self.shape.len() > slice.len() {
            panic!("position to get needs has the same matrix dimension")
        }
        
        let (shape_mask, position_mask) = slice.into_iter().zip(self.shape.clone().into_iter()).fold( 
            (vec![],vec![]), 
            |(mut shape_mask, mut position_mask), (&position, depht )| {
                match position {
                    Index::Value(index) => {
                        if index >= depht {
                            panic!("position out of the range")
                        }
                        
                        shape_mask.push(1);
                        position_mask.push(index);
    
                        (shape_mask, position_mask)
                    },
                    Index::Free => {
                        shape_mask.push(depht);
                        position_mask.push(0);
    
                        (shape_mask, position_mask)
                    }
                }
            }
        );

        MatrixView {matrix: &self, shape_mask, position_mask }
    }
}

fn unhash(id: usize, shape: &Vec<usize>) -> Vec<usize> {
    let (position, _) = shape.iter().rev().fold(
        (vec![], id), 
        |(position, quocient), depht| {
            let new_depht = quocient % depht;
            let new_quocient = quocient/depht;
            
            let new_position = list::concat(vec![new_depht], position);
            (new_position, new_quocient)
        }
    );

    position
}

fn hash(position: Vec<usize>, shape: &Vec<usize>) -> usize {
    if position.len() > shape.len() {
        panic!("position to get needs has the same matrix dimension")
    }

    (0..position.len()).fold(0, |id, index| {
        id*shape[index]+position[index]
    })
}

pub fn zip<T>(matrixs: Vec<Matrix<T>>) -> Matrix<T> 
where T: Copy {
    let all_shapes_equals = (0..matrixs.len()-1).any(
        |i| matrixs[i].shape() != matrixs[i+1].shape()
    );
    
    if all_shapes_equals {
        panic!("all matrixs must have the sames shapes")
    }
    
    let shape = fp::head(matrixs.iter()).unwrap().shape().clone();

    let new_depht = matrixs.len();
    let new_shape = list::concat(vec![new_depht], shape);

    let new_values = matrixs.into_iter().fold(
        vec![],
        |result, m| {
            list::concat(result, m.taken_raw())
        }
    );

    new(new_values, &new_shape)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acess_position() {
        let matrix = new(vec![0,1,2,3,4,5], &vec![2usize, 3usize]);

        assert_eq!(matrix.get(vec![0,0]), 0);
        assert_eq!(matrix.get(vec![0,1]), 1);
        assert_eq!(matrix.get(vec![0,2]), 2);
        assert_eq!(matrix.get(vec![1,0]), 3);
        assert_eq!(matrix.get(vec![1,1]), 4);
        assert_eq!(matrix.get(vec![1,2]), 5);
    }

    #[test]
    fn mask_test() {
        let matrix = new(vec![0,1,2,3,4,5], &vec![2usize, 3usize]);

        let sub_matrix = matrix.view(&[Index::Value(0), Index::Free]);
        assert_eq!(sub_matrix.get(vec![0]), 0);
        assert_eq!(sub_matrix.get(vec![1]), 1);
        assert_eq!(sub_matrix.get(vec![2]), 2);

        let sub_matrix = matrix.view(&[Index::Value(1), Index::Free]);
        assert_eq!(sub_matrix.get(vec![0]), 3);
        assert_eq!(sub_matrix.get(vec![1]), 4);
        assert_eq!(sub_matrix.get(vec![2]), 5);

        let sub_matrix = matrix.view(&[Index::Free, Index::Value(0)]);
        assert_eq!(sub_matrix.get(vec![0]), 0);
        assert_eq!(sub_matrix.get(vec![1]), 3);

        let sub_matrix = matrix.view(&[Index::Free, Index::Value(1)]);
        assert_eq!(sub_matrix.get(vec![0]), 1);
        assert_eq!(sub_matrix.get(vec![1]), 4);

        let sub_matrix = matrix.view(&[Index::Free, Index::Value(2)]);
        assert_eq!(sub_matrix.get(vec![0]), 2);
        assert_eq!(sub_matrix.get(vec![1]), 5);
    }

    #[test]
    fn zip_test() {
        let m1 = new(vec![0,1,2,3,4,5], &vec![2usize, 3usize]);
        let m2 = new(vec![6,7,8,9,10,11], &vec![2usize, 3usize]);

        let ziped = matrix::zip(vec![m1, m2]);
        
        assert_eq!(ziped.get(vec![0,0,0]), 0);
        assert_eq!(ziped.get(vec![0,0,1]), 1);
        assert_eq!(ziped.get(vec![0,0,2]), 2);
        assert_eq!(ziped.get(vec![0,1,0]), 3);
        assert_eq!(ziped.get(vec![0,1,1]), 4);
        assert_eq!(ziped.get(vec![0,1,2]), 5);
        assert_eq!(ziped.get(vec![1,0,0]), 6);
        assert_eq!(ziped.get(vec![1,0,1]), 7);
        assert_eq!(ziped.get(vec![1,0,2]), 8);
        assert_eq!(ziped.get(vec![1,1,0]), 9);
        assert_eq!(ziped.get(vec![1,1,1]), 10);
        assert_eq!(ziped.get(vec![1,1,2]), 11);
    }
}