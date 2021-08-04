use crate::fp;
use super::*;

pub fn new<T: Clone + Copy>(values: Vec<T>, shape_ref: &[usize]) -> Matrix<T> {
    if shape_ref.iter().product::<usize>() != values.len() {
        panic!("shape dosent match with values")
    }
    let shape = shape_ref.to_vec();
    
    Matrix { values, shape }
}

pub fn vec_to_matrix<T: Clone + Copy>(values: Vec<T>) -> Matrix<T> {
    let shape = vec![values.len()];
    
    Matrix { values, shape }
}

pub fn vec2_to_matrix2<T: Clone + Copy>(values: Vec<Vec<T>>) -> Matrix<T> {
    let y_depht = values.len();
    let x_depht = fp::head(values.iter()).unwrap().len();

    if values.iter().any(|v| v.len() != x_depht) {
        panic!("all lines needs have the same lenght")
    }
    
    let new_values = values.into_iter().flatten().collect();

    Matrix { values: new_values, shape: vec![y_depht, x_depht] }
}

pub fn transposed_vec2_to_matrix2<T: Clone + Copy + Default>(values: Vec<Vec<T>>) -> Matrix<T> {
    let y_depht = values.len();
    let x_depht = fp::head(values.iter()).unwrap().len();

    if values.iter().any(|v| v.len() != x_depht) {
        panic!("all lines needs have the same lenght")
    }
    
    let mut new_values = vec![T::default();y_depht*x_depht];
    let mut i = 0usize;
    for x in 0..x_depht {
        for y in 0..y_depht {
            new_values[i] = values[y][x];
            i = i+1;
        }
    }

    Matrix { values: new_values, shape: vec![x_depht, y_depht] }
}

pub fn merge<T: Clone + Copy>(matrixs: Vec<Matrix<T>>) -> Matrix<T> {
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

    new(new_values, &new_shape)
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

pub fn cartesian_product_of_shape(shape: Vec<usize>) -> impl Iterator<Item = Vec<usize>> { 
    if shape.iter().any(|&i| i == 0) {
        panic!("colum need depht >= 1")
    }

    let cartesian_product_lenght = shape.iter().product();
    let sets_len = shape.len();
    let mut indexs_count = vec![0usize;sets_len];
    
    (0..cartesian_product_lenght).map(move |_| {
        let result = indexs_count.clone();

        for i in (0..indexs_count.len()).rev() {
            
            if indexs_count[i] == shape[i] - 1 {
                indexs_count[i] = 0;

                if i == 0 {
                    break;
                }
            } else {
                indexs_count[i] = indexs_count[i] + 1;
                break;
            } 
        }

        result
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
    fn zip_test() {
        let m1 = new(vec![0,1,2,3,4,5], &[2usize, 3usize]);
        let m2 = new(vec![6,7,8,9,10,11], &[2usize, 3usize]);

        let ziped = matrix::merge(vec![m1, m2]);
        
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

    #[test]
    fn cartesian_product_of_shape_test() {
        let mut cartesian_product = cartesian_product_of_shape(vec![1,2,3]);

        assert_eq!(cartesian_product.next().unwrap(), vec![0,0,0]);
        assert_eq!(cartesian_product.next().unwrap(), vec![0,0,1]);
        assert_eq!(cartesian_product.next().unwrap(), vec![0,0,2]);
        assert_eq!(cartesian_product.next().unwrap(), vec![0,1,0]);
        assert_eq!(cartesian_product.next().unwrap(), vec![0,1,1]);
        assert_eq!(cartesian_product.next().unwrap(), vec![0,1,2]);
        assert_eq!(cartesian_product.next(), None);
    }

    #[test]
    fn create_transposed() {
        let matrix = transposed_vec2_to_matrix2(
            vec![
                vec![0,3],
                vec![1,4],
                vec![2,5],
                ]
            );

        assert_eq!(matrix.get(&[0,0]), &0);
        assert_eq!(matrix.get(&[0,1]), &1);
        assert_eq!(matrix.get(&[0,2]), &2);
        assert_eq!(matrix.get(&[1,0]), &3);
        assert_eq!(matrix.get(&[1,1]), &4);
        assert_eq!(matrix.get(&[1,2]), &5);
    }
}