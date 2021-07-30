use ndarray::Array;
use optical_waveguide::{export, fp::Matrix};
use structopt::StructOpt;
use optical_waveguide::tools;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input_file: std::path::PathBuf,
    output_file: String,
}

fn main() {
    let args = Cli::from_args();

    let option_file = hdf5::File::open(args.input_file);

    match option_file {
        Ok(file) => {
            let output = hdf5::File::create(args.output_file).unwrap();

            let intensity_norm = normalize(&file, "intensity");
            let intensity_shape = intensity_norm.shape().to_vec();
            export::save_surface(&output, intensity_norm.into_raw(), intensity_shape, "intensity");

            let eletric_field_norm = normalize(&file, "eletric_field");
            let eletric_field_shape = eletric_field_norm.shape().to_vec();
            export::save_surface(&output, eletric_field_norm.into_raw(), eletric_field_shape, "eletric_field");
            
            copy_dataset(&output, &file, "deltas");
            copy_dataset(&output, &file, "core");

        } Err(_) => {
            println!("Cant open first file");
        }
    }    
}

fn normalize(file: &hdf5::File, dataset_name: &str) -> Matrix<f64> {
    let dataset = file.dataset(dataset_name).unwrap();

    tools::normalize(tools::dataset_to_matrix(dataset))
}

fn copy_dataset(output: &hdf5::File, file: &hdf5::File, dataset_name: &str) {
    let dataset_option = file.dataset(dataset_name);
    
    match dataset_option {
        Ok(dataset) => {
            let array = Array::from_shape_vec(dataset.shape(), dataset.read_raw::<f64>().unwrap()).unwrap();
            let dataset_copy = output.new_dataset::<f64>().create(dataset_name, dataset.shape()).unwrap();
            dataset_copy.write(&array).unwrap();
        }
        Err(_) => {
            println!("Havent {} dataset", dataset_name);
        }
    }
}