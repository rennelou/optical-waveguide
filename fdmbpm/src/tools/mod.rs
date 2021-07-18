pub fn normalize(values: Vec<f64>, shape: Vec<usize>) -> (Vec<f64>, Vec<usize>) {
    
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