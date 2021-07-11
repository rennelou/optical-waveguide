use ndarray::Array;
use structopt::StructOpt;
use std::cmp;

#[derive(StructOpt)]
struct CompareArgs {
    #[structopt(parse(from_os_str))]
    reference: std::path::PathBuf,
    data: std::path::PathBuf,
    output_name: String,
}

fn main() {
    let args = CompareArgs::from_args();

    let option_file_1 = hdf5::File::open(args.reference);
    let option_file_2 = hdf5::File::open(args.data);

    match option_file_1 {
        Ok(file1) => {
            match option_file_2 {
                Ok(file2) => {
                    let output = hdf5::File::create(args.output_name).unwrap();

                    let reference_areas = areas(&file1);
                    let data_areas = areas(&file2);
                    let diff_area = areas_diff(&reference_areas, &data_areas);
                    
                    save_line(&output, reference_areas, "areas_reference");
                    save_line(&output, data_areas, "areas_data");
                    save_line(&output, diff_area, "areas_diff");
                    
                    save_diff(&output, formeted_diffs(&file1, &file2));
                } Err(_) => {
                    println!("Cant open second file");    
                }
            }
        } Err(_) => {
            println!("Cant open first file");
        }
    }
}

fn areas_diff(reference: &Vec<f64>, data: &Vec<f64>) -> Vec<f64> {
    let depht = cmp::min(reference.len(), data.len());

    let diffs = (0..depht).map(|i| (reference[i] - data[i]).abs()).collect();

    diffs
}

fn areas(file1: &hdf5::File) -> Vec<f64> {
    let dataset1 = file1.dataset("intensity").unwrap();

    let shape1 = dataset1.shape();

    if shape1.len() == 2 {
        let data1 = dataset1.read_raw::<f64>().unwrap();

        let depht0 = shape1[0];
        let depht1 = shape1[1];

        let result: Vec<_> = (0..depht0).map(|i| {
            
            let area = (0..depht1)
                .map(|j| data1[i*depht1 + j]).sum();
            
            area
        }).collect();

        return result;
    } else {
        panic!("Both datasets needs has depht two");
    }   
}

fn formeted_diffs(file1: &hdf5::File, file2: &hdf5::File) -> (Vec<f64>, usize, usize) {
    let (diffs, depht0, depht1) = diffs(&file1, &file2);

    (diffs.into_iter().flatten().collect(), depht0, depht1)
}

fn diffs(file1: &hdf5::File, file2: &hdf5::File) -> (Vec<Vec<f64>>, usize, usize) {
    let dataset1 = file1.dataset("intensity").unwrap();
    let dataset2 = file2.dataset("intensity").unwrap();

    let shape1 = dataset1.shape();
    let shape2 = dataset2.shape();

    if shape1.len() == 2 && shape2.len() == 2 {
        let data1 = dataset1.read_raw::<f64>().unwrap();
        let data2 = dataset2.read_raw::<f64>().unwrap();

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

fn save_diff(output: &hdf5::File, (data, depht0, depht1):(Vec<f64>, usize, usize)) {
    let shape = vec![depht0, depht1];
    let dataset = output.new_dataset::<f64>().create("diff", shape.clone()).unwrap();
    let result_array = Array::from_shape_vec(shape.clone(), data).unwrap();

    dataset.write(&result_array).unwrap();
}

fn save_line(output: &hdf5::File, result: Vec<f64>, title: &str) {
    
    let dataset = output.new_dataset::<f64>().create(title, result.len()).unwrap();
    let result_array = Array::from_shape_vec(result.len(), result).unwrap();

    dataset.write(&result_array).unwrap();
}