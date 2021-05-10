use ndarray::Array;
use crate::fp::List;
use crate::waveguide::Intensity;

pub fn hdf5<const N: usize>(title: &str, intensity: Intensity<N>) {
    if let [zdelta, xdelta] = intensity.deltas[0..1] {
        if let [zsteps, xsteps] = intensity.shape[0..1] {
            hdf5_2d(title, intensity.values, (zsteps, xsteps), [zdelta, xdelta]);

            return;
        }
    }
    
    panic!("dimension dont match");
}

pub fn hdf5_2d(title: &str, values: List<f64>, shape: (usize, usize), deltas: [f64;2]) {
    let array = Array::from_shape_vec(shape, values).unwrap();
    
    let file = hdf5::File::create(title).unwrap();
    let group = file.create_group("dir").unwrap();
    
    let deltas_hdf5 = group.new_dataset::<f64>().create("deltas", 2).unwrap();
    deltas_hdf5.write(&deltas).unwrap();
    
    let dataset = group.new_dataset::<f64>().create("intensity", shape).unwrap();
    dataset.write(&array).unwrap();
}