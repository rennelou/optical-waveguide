use ndarray::Array;
use crate::{fp::{matrix}, waveguide::{EletricField, cores::Core}};

pub fn hdf5<const D: usize>(title: &str, eletric_field: &EletricField, core: &impl Core<D>) {
   
    let shape = eletric_field.shape();
    let deltas = eletric_field.grid_steps();
    
    let intensity = eletric_field.get_intensity();
    let array = Array::from_shape_vec(shape.clone(), intensity).unwrap();
    
    let file = hdf5::File::create(title).unwrap();
    let group = file.create_group("dir").unwrap();
    
    let deltas_dataset = group.new_dataset::<f64>().create("deltas", deltas.len()).unwrap();
    deltas_dataset.write(deltas).unwrap();
    
    let dataset = group.new_dataset::<f64>().create("intensity", shape.clone()).unwrap();
    dataset.write(&array).unwrap();

    let core_values = Array::from_shape_vec(shape.clone(), get_core_matrix(core)).unwrap();
    let core_dataset = group.new_dataset::<f64>().create("core", shape.clone()).unwrap();
    core_dataset.write(&core_values).unwrap();
}

fn get_core_matrix<const D: usize>(core: &impl Core<D>) -> Vec<f64> {
    let shape = core.get_shape().to_vec();
    
    (0..shape.iter().product()).map(|id| {
        let position = matrix::id_to_position(id, &shape);
		core.get_n(position.as_slice(), core.get_n0())

	}).collect()
}
