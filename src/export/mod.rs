use ndarray::Array;
use crate::waveguide::eletric_field_2d::Intensity;

pub fn hdf5(title: &str, intensity: Intensity) {
    let (xdelta, zdelta) = intensity.deltas;

    let array = Array::from_shape_vec(intensity.shape, intensity.values).unwrap();

    let file = hdf5::File::create(title).unwrap();
    let group = file.create_group("dir").unwrap();
    
    let deltas_hdf5 = group.new_dataset::<f64>().create("deltas", 2).unwrap();
    deltas_hdf5.write(&[xdelta, zdelta]).unwrap();

    let dataset = group.new_dataset::<f64>().create("intensity", intensity.shape).unwrap();
    dataset.write(&array).unwrap();
}
    