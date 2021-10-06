use ndarray::Array;
use crate::{fp::Matrix};

pub fn hdf5(title: &str, shape: &[usize], deltas: &[f64], eletric_field: Matrix<f64>, intensity: Matrix<f64>, core_matrix: Vec<f64>) {
   
    let file = hdf5::File::create(title).unwrap();
    
    save_instensity(&file, intensity);
    save_eletric_fields(&file, eletric_field);
    save_deltas(&file, deltas.to_vec(), vec![deltas.len()]);
    
    save_core(&file, core_matrix, shape.to_vec());
}

fn save_instensity(output: &hdf5::File, data: Matrix<f64>) {
    let shape = data.shape().to_vec();
    save_surface(output, data.into_raw(), shape, "intensity")
}

fn save_eletric_fields(output: &hdf5::File, data: Matrix<f64>) {
    let shape = data.shape().to_vec();
    save_surface(output, data.into_raw(), shape, "eletric_field")
}

fn save_deltas(output: &hdf5::File, data: Vec<f64>, shape: Vec<usize>) {
    save_surface(output, data, shape, "deltas")
}

fn save_core(output: &hdf5::File, data: Vec<f64>, shape: Vec<usize>) {
    save_surface(output, data, shape, "core")
}

pub fn save_surface(output: &hdf5::File, data: Vec<f64>, shape: Vec<usize>, title: &str) {
    let dataset = output.new_dataset::<f64>().create(title, shape.clone()).unwrap();
    let result_array = Array::from_shape_vec(shape.clone(), data).unwrap();

    dataset.write(&result_array).unwrap();
}
