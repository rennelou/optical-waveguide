use crate::functional_types::Matrix;
use crate::simulator::types::SimulationResult;
use ndarray::Array;

pub fn hdf5(title: &str, results: SimulationResult) {
   
    let file = hdf5::File::create(title).unwrap();

    save_instensity(&file, results.intensity);
    save_eletric_fields(&file, results.eletric_field);

    let grid_steps_len = results.grid_steps.len();
    save_deltas(&file, results.grid_steps, vec![grid_steps_len]);
    
    save_core(&file, results.refractive_indexes, results.shape);
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
    let dataset = output.new_dataset::<f64>().shape(shape.as_slice()).create(title).unwrap();
    let result_array = Array::from_shape_vec(shape.clone(), data).unwrap();

    dataset.write(&result_array).unwrap();
}
