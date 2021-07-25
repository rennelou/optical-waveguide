use ndarray::Array;
use crate::{fp::{matrix}, waveguide::{EletricField, cores::Core}};

pub fn hdf5<const D: usize>(title: &str, eletric_field: &EletricField<D>, core: &impl Core<D>) {
   
    let file = hdf5::File::create(title).unwrap();

    let shape = eletric_field.shape();
    
    save_instensity(&file, eletric_field.get_intensity(), shape.to_vec());
    save_eletric_fields(&file, eletric_field.get_values(), shape.to_vec());
    
    let deltas = eletric_field.grid_steps();
    save_deltas(&file, deltas.to_vec(), vec![deltas.len()]);
    
    save_core(&file, get_core_matrix(core), shape.to_vec());
}

fn save_instensity(output: &hdf5::File, data: Vec<f64>, shape: Vec<usize>) {
    save_surface(output, data, shape, "intensity")
}

fn save_eletric_fields(output: &hdf5::File, data: Vec<f64>, shape: Vec<usize>) {
    save_surface(output, data, shape, "eletric_field")
}

fn save_deltas(output: &hdf5::File, data: Vec<f64>, shape: Vec<usize>) {
    save_surface(output, data, shape, "deltas")
}

fn save_core(output: &hdf5::File, data: Vec<f64>, shape: Vec<usize>) {
    save_surface(output, data, shape, "core")
}

fn get_core_matrix<const D: usize>(core: &impl Core<D>) -> Vec<f64> {
    let shape = core.get_shape().to_vec();
    
    (0..shape.iter().product()).map(|id| {
        let position = matrix::id_to_position(id, &shape);
		core.get_n(position.as_slice(), core.get_n0())

	}).collect()
}

pub fn save_surface(output: &hdf5::File, data: Vec<f64>, shape: Vec<usize>, title: &str) {
    let dataset = output.new_dataset::<f64>().create(title, shape.clone()).unwrap();
    let result_array = Array::from_shape_vec(shape.clone(), data).unwrap();

    dataset.write(&result_array).unwrap();
}
