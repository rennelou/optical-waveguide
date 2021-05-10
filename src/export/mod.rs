use ndarray::Array;
use ndarray::IxDyn;
use crate::waveguide::EletricField;

pub fn hdf5<const N: usize>(title: &str, eletric_field: &EletricField<N>) {
   
    let shape = eletric_field.shape;
    let deltas = eletric_field.deltas;
    
    let intensity = eletric_field.get_intensity();
    let array = Array::from_shape_vec(IxDyn(&shape), intensity).unwrap();
    
    let file = hdf5::File::create(title).unwrap();
    let group = file.create_group("dir").unwrap();
    
    let deltas_hdf5 = group.new_dataset::<f64>().create("deltas", 2).unwrap();
    deltas_hdf5.write(&deltas).unwrap();
    
    let dataset = group.new_dataset::<f64>().create("intensity", shape.to_vec()).unwrap();
    dataset.write(&array).unwrap();
}
