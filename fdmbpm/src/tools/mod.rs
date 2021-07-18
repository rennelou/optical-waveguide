use std::cmp;

pub fn normalize((values, shape): (Vec<f64>, Vec<usize>)) -> (Vec<f64>, Vec<usize>) {
    
    if shape.len() == 2 {
        let depht0 = shape[0];
        let depht1 = shape[1];

        let area_input =(0..depht1).map(|j| values[j]).sum::<f64>();
        let new_values = values.into_iter().map(|x| x / area_input).collect();

        return (new_values, vec![depht0, depht1]);
    } else {
        panic!("Both datasets needs has depht two");
    }
}

pub fn areas_diff((data1, shape1): (Vec<f64>, Vec<usize>), (data2, shape2): (Vec<f64>, Vec<usize>)) -> Vec<f64> {
    let (diffs, _, _) = diffs(data1, data2, shape1, shape2);

    let diff_sums = diffs.into_iter().map(|diffs_vec| diffs_vec.into_iter().sum()).collect();

    diff_sums
}

fn diffs(data1: Vec<f64>, data2: Vec<f64>, shape1: Vec<usize>, shape2: Vec<usize>) -> (Vec<Vec<f64>>, usize, usize) {
    if shape1.len() == 2 && shape2.len() == 2 {
        
        let depht0 = cmp::min(shape1[0], shape2[0]);
        let depht1 = cmp::min(shape1[1], shape2[1]);

        let result: Vec<_> = (0..depht0).map(|i| {
            
            let diffs: Vec<_> = (0..depht1)
                .map(|j| (data1[i*depht1 + j] - data2[i*depht1 + j]).abs()).collect();
            
            diffs
        }).collect();

        return (result, depht0, depht1);
    } else {
        panic!("Both datasets needs has depht two");
    }   
}

pub fn dataset_to_matrix(dataset: hdf5::Dataset) -> (Vec<f64>, Vec<usize>) {
    let data = dataset.read_raw::<f64>().unwrap();
    let shape = dataset.shape();

    (data, shape)
}