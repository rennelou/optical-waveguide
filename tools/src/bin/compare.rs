use ndarray::Array;
use structopt::StructOpt;
use std::cmp;

#[derive(StructOpt)]
struct CompareArgs {
    #[structopt(parse(from_os_str))]
    h5_1: std::path::PathBuf,
    h5_2: std::path::PathBuf,
    output_name: String,
}

fn main() {
    let args = CompareArgs::from_args();

    let option_file_1 = hdf5::File::open(args.h5_1);
    let option_file_2 = hdf5::File::open(args.h5_2);

    match option_file_1 {
        Ok(file1) => {
            match option_file_2 {
                Ok(file2) => {
                    let output = hdf5::File::create(args.output_name).unwrap();

                    let result = average_error(&file1, &file2);

                    save_diff(&output, formeted_diff(&file1, &file2));
                    save_average_error(&output, result);
                } Err(_) => {
                    println!("Cant open second file");    
                }
            }
        } Err(_) => {
            println!("Cant open first file");
        }
    }
}

fn formeted_diff(file1: &hdf5::File, file2: &hdf5::File) -> (Vec<f64>, usize, usize) {
    let (diffs, depht0, depht1) = diff(&file1, &file2);

    (diffs.into_iter().flatten().collect(), depht0, depht1)
}

fn diff(file1: &hdf5::File, file2: &hdf5::File) -> (Vec<Vec<f64>>, usize, usize) {
    let dataset1 = file1.dataset("eletric_field").unwrap();
    let dataset2 = file2.dataset("eletric_field").unwrap();

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

fn average_error(file1: &hdf5::File, file2: &hdf5::File) -> Vec<f64> {
    let (diffs, _, _) = diff(&file1, &file2);
    
    let average = diffs.iter().map(
        |diff| diff.iter().sum::<f64>() / (diff.len() as f64)
    ).collect();
    
    average
}

fn save_diff(output: &hdf5::File, (data, depht0, depht1):(Vec<f64>, usize, usize)) {
    let shape = vec![depht0, depht1];
    let dataset = output.new_dataset::<f64>().create("diff", shape.clone()).unwrap();
    let result_array = Array::from_shape_vec(shape.clone(), data).unwrap();

    dataset.write(&result_array).unwrap();
}

fn save_average_error(output: &hdf5::File, result: Vec<f64>) {
    
    let dataset = output.new_dataset::<f64>().create("avarege_error", result.len()).unwrap();
    let result_array = Array::from_shape_vec(result.len(), result).unwrap();

    dataset.write(&result_array).unwrap();
}