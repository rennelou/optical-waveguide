use std::cmp;
use crate::fp::{matrix};

pub fn normalize((values, shape): (Vec<f64>, Vec<usize>)) -> (Vec<f64>, Vec<usize>) {
    let area_input = submatrix((values.clone(), shape.clone()), 0).iter().sum::<f64>();
    
    let new_values = values.into_iter().map(|x| x / area_input).collect();

    return (new_values, shape);
}

pub fn areas_diff((data1, shape1): (Vec<f64>, Vec<usize>), (data2, shape2): (Vec<f64>, Vec<usize>)) -> Vec<f64> {
    let (diffs, _, _) = diffs(data1, data2, shape1, shape2);

    let diff_sums = diffs.into_iter().map(|diffs_vec| diffs_vec.into_iter().sum()).collect();

    diff_sums
}

fn diffs(data1: Vec<f64>, data2: Vec<f64>, shape1: Vec<usize>, shape2: Vec<usize>) -> (Vec<Vec<f64>>, usize, Vec<usize>) {
    if shape1.len() == shape2.len() {

        let depht0 = cmp::min(shape1[0], shape2[0]);
        
        let sub_shape1 = shape1[1..].to_vec();
        let sub_shape2 = shape2[1..].to_vec();
        
        if sub_shape1.iter().zip(sub_shape2.iter()).all(|(d1,d2)| d1 == d2) {
            
            let result = (0..depht0).map(|z| {
                let sub_data1 = submatrix((data1.clone(), shape1.clone()), z);
                let sub_data2 = submatrix((data2.clone(), shape2.clone()), z);

                sub_data1.into_iter().zip(sub_data2.into_iter()).map(
                    |(d1, d2)| (d1 - d2).abs()
                ).collect()

            }).collect();

            return (result, depht0, sub_shape1);
        }
    } 
    
    panic!("datasets needs has transversal grid equals")
}

pub fn dataset_to_matrix(dataset: hdf5::Dataset) -> (Vec<f64>, Vec<usize>) {
    let data = dataset.read_raw::<f64>().unwrap();
    let shape = dataset.shape();

    (data, shape)
}

fn submatrix((values, shape): (Vec<f64>, Vec<usize>), z: usize) -> Vec<f64> {
    let sub_shape = shape[1..].to_vec();

    cartesian_product(sub_shape).into_iter().map(
        |mut position| {
            position.insert(0, z);
            values[matrix::position_to_id(position.as_slice(), shape.as_slice())].clone()
        }
    ).collect()
}

fn cartesian_product(shape: Vec<usize>) -> Vec<Vec<usize>> {
    (0..shape.iter().product()).map(
        |id| matrix::id_to_position(id, shape.as_slice())
    ).collect()
}