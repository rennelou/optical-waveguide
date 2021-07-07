use ndarray::Array;
use structopt::StructOpt;
use std::cmp;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    h5_1: std::path::PathBuf,
    h5_2: std::path::PathBuf,
    output_name: String,
}

fn main() {
    let args = Cli::from_args();

    let option_file_1 = hdf5::File::open(args.h5_1);
    let option_file_2 = hdf5::File::open(args.h5_2);

    match option_file_1 {
        Ok(file1) => {
            match option_file_2 {
                Ok(file2) => {
                    
                    let result = compare(file1, file2);

                    save_result(args.output_name, result);
                } Err(_) => {
                    println!("Cant open second file");    
                }
            }
        } Err(_) => {
            println!("Cant open first file");
        }
    }
}

fn compare(file1: hdf5::File, file2: hdf5::File) -> Vec<f64> {
    let dataset1 = file1.dataset("intensity").unwrap();
    let dataset2 = file2.dataset("intensity").unwrap();

    let shape1 = dataset1.shape();
    let shape2 = dataset2.shape();

    if shape1.len() == 2 && shape2.len() == 2 {
        let data1 = dataset1.read_raw::<f64>().unwrap();
        let data2 = dataset2.read_raw::<f64>().unwrap();

        let depht0 = cmp::min(shape1[0], shape2[0]);
        let result: Vec<_> = (0..depht0).map(|i| {
            
            let diffs: Vec<_> = (0..(cmp::min(shape1[1], shape2[1])))
                .map(|j| (data1[i*depht0 + j] - data2[i*depht0 + j]).abs()).collect();
            
                let average = diffs.iter().sum::<f64>() / (diffs.len() as f64);

            average
        }).collect();

        return result;
    } else {
        panic!("Both datasets needs has depht two");
    }   
}

fn save_result(output_name: String, result: Vec<f64>) {
    let output = hdf5::File::create(output_name).unwrap();
    let dataset = output.new_dataset::<f64>().create("avarege_error", result.len()).unwrap();
    let result_array = Array::from_shape_vec(result.len(), result).unwrap();

    dataset.write(&result_array).unwrap();
}