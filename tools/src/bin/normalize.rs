use ndarray::Array;
use structopt::StructOpt;

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

            let intensity_norm_surface = normalize(&file, "intensity");
            tools::save_surface(&output, intensity_norm_surface, "intensity");

            let eletric_field_norm_surface = normalize(&file, "eletric_field");
            tools::save_surface(&output, eletric_field_norm_surface, "eletric_field");
            
            copy_dataset(&output, &file, "deltas");
            copy_dataset(&output, &file, "core");

        } Err(_) => {
            println!("Cant open first file");
        }
    }    
}

fn normalize(file: &hdf5::File, dataset_name: &str) -> (Vec<f64>, usize, usize) {
    let dataset = file.dataset(dataset_name).unwrap();

    let values = dataset.read_raw::<f64>().unwrap();
    let shape = dataset.shape();

    if shape.len() == 2 {
        let depht0 = shape[0];
        let depht1 = shape[1];

        let area_input =(0..depht1).map(|j| values[j]).sum::<f64>();
        let new_values = values.into_iter().map(|x| x / area_input).collect();

        return (new_values, depht0, depht1);
    } else {
        panic!("Both datasets needs has depht two");
    }
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