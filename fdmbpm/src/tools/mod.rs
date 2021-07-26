use std::cmp;
use crate::fp::{Matrix, matrix};

pub fn normalize(m: Matrix<f64>) -> Matrix<f64> {
    let area_input = submatrix(&m, 0).iter().sum::<f64>();
    
    let new_values = m.raw().into_iter().map(|x| x / area_input).collect();

    return matrix::new(new_values, m.shape());
}

pub fn areas_diff(m1: Matrix<f64>, m2: Matrix<f64>) -> Vec<f64> {
    let (diffs, _, _) = diffs(m1, m2);

    let diff_sums = diffs.into_iter().map(|diffs_vec| diffs_vec.into_iter().sum()).collect();

    diff_sums
}

fn diffs(m1: Matrix<f64>, m2: Matrix<f64>) -> (Vec<Vec<f64>>, usize, Vec<usize>) {
    let shape1 = m1.shape();
    let shape2 = m2.shape();

    if shape1.len() == shape2.len() {

        let depht0 = cmp::min(shape1[0], shape2[0]);
        
        let sub_shape1 = shape1[1..].to_vec();
        let sub_shape2 = shape2[1..].to_vec();
        
        if sub_shape1.iter().zip(sub_shape2.iter()).all(|(d1,d2)| d1 == d2) {
            
            let result = (0..depht0).map(|z| {
                let sub_data1 = submatrix(&m1, z);
                let sub_data2 = submatrix(&m2, z);

                sub_data1.into_iter().zip(sub_data2.into_iter()).map(
                    |(d1, d2)| (d1 - d2).abs()
                ).collect()

            }).collect();

            return (result, depht0, sub_shape1);
        }
    } 
    
    panic!("datasets needs has transversal grid equals")
}

pub fn dataset_to_matrix(dataset: hdf5::Dataset) -> Matrix<f64> {
    let data = dataset.read_raw::<f64>().unwrap();
    let shape = dataset.shape();

    matrix::new(data, shape.as_slice())
}

fn submatrix(m: &Matrix<f64>, z: usize) -> Vec<f64> {
    let shape = m.shape();
    let sub_shape = shape[1..].to_vec();

    matrix::dephts_cartesian_product(sub_shape).into_iter().map(
        |mut position| {
            position.insert(0, z);
            
            m.get(position.as_slice()).clone()
        }
    ).collect()
}