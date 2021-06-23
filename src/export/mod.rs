use ndarray::Array;
use crate::{fp::{matrix}, waveguide::{EletricField, cores::Core}};

pub fn hdf5<const D: usize>(title: &str, eletric_field: &EletricField, core: &impl Core<D>) {
   
    let shape = eletric_field.shape();
    let deltas = eletric_field.grid_steps();
    
    let file = hdf5::File::create(title).unwrap();
    
    let deltas_dataset = file.new_dataset::<f64>().create("deltas", deltas.len()).unwrap();
    deltas_dataset.write(deltas).unwrap();
    
    let eletric_field_array = Array::from_shape_vec(shape.clone(), eletric_field.get_values()).unwrap();
    let eletric_field_dataset = file.new_dataset::<f64>().create("eletric_field", shape.clone()).unwrap();
    eletric_field_dataset.write(&eletric_field_array).unwrap();

    let intensity_array = Array::from_shape_vec(shape.clone(), eletric_field.get_intensity()).unwrap();
    let dataset = file.new_dataset::<f64>().create("intensity", shape.clone()).unwrap();
    dataset.write(&intensity_array).unwrap();

    let core_values = Array::from_shape_vec(shape.clone(), get_core_matrix(core)).unwrap();
    let core_dataset = file.new_dataset::<f64>().create("core", shape.clone()).unwrap();
    core_dataset.write(&core_values).unwrap();
}

fn get_core_matrix<const D: usize>(core: &impl Core<D>) -> Vec<f64> {
    let shape = core.get_shape().to_vec();
    
    (0..shape.iter().product()).map(|id| {
        let position = matrix::id_to_position(id, &shape);
		core.get_n(position.as_slice(), core.get_n0())

	}).collect()
}
