use crate::fp;
use super::*;

pub mod view;

#[derive(Clone, Copy)]
pub enum Index {
    Free,
    Value(usize)
}

pub struct MatrixView<'a, T: Copy, const D: usize> {
    matrix: &'a Matrix<T>,
    shape_mask: Vec<usize>,
    position_mask: Vec<usize>
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
        dimension(self.shape())
    }

    pub fn get(&self, position: &[usize]) -> &T {
        &self.values[hash(position, self.shape())]
    }

    pub fn view<const D: usize>(&self, slice: &[Index]) -> MatrixView<T, D> {
        let slice_dimension = slice_dimension(slice);
        
        if slice_dimension != D {
            panic!("slice dosent match with dimension of view")
        }

        if self.dimension() < slice_dimension {
            panic!("slice dimension must be less or equal than dimension matrix")
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
    
        MatrixView { matrix: &self, shape_mask, position_mask }
    }
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

fn dimension(shape: &Vec<usize>) -> usize {
    shape.iter().copied().fold(
        0, 
        |dim, depht|{ if depht > 1 {dim + 1} else {dim} }
    )
}

fn slice_dimension(shape: &[Index]) -> usize {
    shape.iter().copied().fold(
        0, 
        |dim, index|
            match index {
                Index::Value(_) => { dim }
                Index::Free => { dim + 1 }
            }
    )
}

fn unhash(id: usize, shape: &[usize]) -> Vec<usize> {
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

fn hash(position: &[usize], shape: &[usize]) -> usize {
    (0..position.len()).fold(0, |id, index| {
        id*shape[index]+position[index]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acess_position() {
        let matrix = new(vec![0,1,2,3,4,5], &vec![2usize, 3usize]);

        assert_eq!(matrix.get(&[0,0]), &0);
        assert_eq!(matrix.get(&[0,1]), &1);
        assert_eq!(matrix.get(&[0,2]), &2);
        assert_eq!(matrix.get(&[1,0]), &3);
        assert_eq!(matrix.get(&[1,1]), &4);
        assert_eq!(matrix.get(&[1,2]), &5);
    }

    #[test]
    fn zip_test() {
        let m1 = new(vec![0,1,2,3,4,5], &vec![2usize, 3usize]);
        let m2 = new(vec![6,7,8,9,10,11], &vec![2usize, 3usize]);

        let ziped = matrix::zip(vec![m1, m2]);
        
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