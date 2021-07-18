use ndarray::Array;
use structopt::StructOpt;
use rust_fdmbpm::tools;

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
                    let diff_area = areas_diff(&file1, &file2);
                    
                    save_line(&output, reference_areas, "areas_reference");
                    save_line(&output, data_areas, "areas_data");
                    save_line(&output, diff_area, "areas_diff");
                } Err(_) => {
                    println!("Cant open second file");    
                }
            }
        } Err(_) => {
            println!("Cant open first file");
        }
    }
}

fn areas(file: &hdf5::File) -> Vec<f64> {
    let dataset = file.dataset("intensity").unwrap();

    let shape = dataset.shape();

    if shape.len() == 2 {
        let data = dataset.read_raw::<f64>().unwrap();

        let depht0 = shape[0];
        let depht1 = shape[1];

        let result: Vec<_> = (0..depht0).map(|i| {
            
            let area = (0..depht1)
                .map(|j| data[i*depht1 + j]).sum();
            
            area
        }).collect();

        return result;
    } else {
        panic!("Both datasets needs has depht two");
    }   
}

fn areas_diff(file1: &hdf5::File, file2: &hdf5::File) -> Vec<f64> {
    
    let dataset1 = file1.dataset("intensity").unwrap();
    let dataset2 = file2.dataset("intensity").unwrap();

    let data1 = dataset1.read_raw::<f64>().unwrap();
    let data2 = dataset2.read_raw::<f64>().unwrap();

    let shape1 = dataset1.shape();
    let shape2 = dataset2.shape();


    let diff_sums = tools::areas_diff(data1, data2, shape1, shape2);

    diff_sums
}

fn save_line(output: &hdf5::File, result: Vec<f64>, title: &str) {
    
    let dataset = output.new_dataset::<f64>().create(title, result.len()).unwrap();
    let result_array = Array::from_shape_vec(result.len(), result).unwrap();

    dataset.write(&result_array).unwrap();
}