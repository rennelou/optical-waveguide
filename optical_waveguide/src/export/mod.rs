use ndarray::Array;
use crate::{fdmbpm::{cores::Core, eletric_field::EletricField, grid::Grid}, fp::{Matrix, matrix}};

pub fn hdf5<const D: usize>(title: &str, eletric_field: &EletricField, grid: &Grid<D>, core: &impl Core<D>) {
   
    let file = hdf5::File::create(title).unwrap();

    let shape = eletric_field.shape();
    
    save_instensity(&file, eletric_field.get_intensity());
    save_eletric_fields(&file, eletric_field.get_eletric_fields());
    
    let deltas = eletric_field.grid_steps();
    save_deltas(&file, deltas.to_vec(), vec![deltas.len()]);
    
    save_core(&file, get_core_matrix(grid, core), shape.to_vec());
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

fn get_core_matrix<const D: usize>(grid: &Grid<D>, core: &impl Core<D>) -> Vec<f64> {
    let shape = grid.get_shape().to_vec();
    
    matrix::cartesian_product_of_shape(shape).map(
        |position| core.get_n(&grid, position.as_slice(), core.get_n0())
    ).collect()
}

pub fn save_surface(output: &hdf5::File, data: Vec<f64>, shape: Vec<usize>, title: &str) {
    let dataset = output.new_dataset::<f64>().create(title, shape.clone()).unwrap();
    let result_array = Array::from_shape_vec(shape.clone(), data).unwrap();

    dataset.write(&result_array).unwrap();
}
