use ndarray::Array;

pub fn save_surface(output: &hdf5::File, (data, depht0, depht1):(Vec<f64>, usize, usize), title: &str) {
    let shape = vec![depht0, depht1];
    let dataset = output.new_dataset::<f64>().create(title, shape.clone()).unwrap();
    let result_array = Array::from_shape_vec(shape.clone(), data).unwrap();

    dataset.write(&result_array).unwrap();
}