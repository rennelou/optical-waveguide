use ndarray::Array;
use crate::waveguide::EletricField;

pub fn hdf5(title: &str, eletric_field: &EletricField) {
   
    let shape = eletric_field.shape();
    let deltas = eletric_field.grid_steps();
    
    let intensity = eletric_field.get_intensity();
    let array = Array::from_shape_vec(shape.clone(), intensity).unwrap();
    
    let file = hdf5::File::create(title).unwrap();
    let group = file.create_group("dir").unwrap();
    
    let deltas_hdf5 = group.new_dataset::<f64>().create("deltas", deltas.len()).unwrap();
    deltas_hdf5.write(deltas).unwrap();
    
    let dataset = group.new_dataset::<f64>().create("intensity", shape.clone()).unwrap();
    dataset.write(&array).unwrap();
}
