use ndarray::Array;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    output_name: String,
}

fn main() {
    let args = Cli::from_args();

    let result = hdf5::File::open(args.path);
    match result {
        Ok(file_input) => {
            let intensity_dataset = file_input.dataset("intensity").unwrap();
    
            let intensity_values = intensity_dataset.read_raw::<f64>().unwrap();
            let shape = intensity_dataset.shape();

            if shape.len() == 2 {
                let max = intensity_values.iter().cloned().fold(f64::NAN, f64::max);
                let min = intensity_values.iter().cloned().fold(f64::NAN, f64::min);
            
                let new_values: Vec<_> = intensity_values.into_iter().map(
                    |x| (x-min)/(max-min)
                ).collect();
            
                let new_values_array = Array::from_shape_vec(shape.clone(), new_values).unwrap();
                let file_output = hdf5::File::create(args.output_name).unwrap();
                let dataset = file_output.new_dataset::<f64>().create("intensity", shape.clone()).unwrap();
                dataset.write(&new_values_array).unwrap();
            
                // Copy Core and Deltas
                let deltas_dataset = file_input.dataset("deltas").unwrap();
                let deltas_array = Array::from_shape_vec(deltas_dataset.shape(), deltas_dataset.read_raw::<f64>().unwrap()).unwrap();
                let deltas_dataset = file_output.new_dataset::<f64>().create("deltas", deltas_dataset.shape()).unwrap();
                deltas_dataset.write(&deltas_array).unwrap();
            
                let core_dataset = file_input.dataset("core");
                match core_dataset {
                    Ok(dataset) => {
                        let core_array = Array::from_shape_vec(dataset.shape(), dataset.read_raw::<f64>().unwrap()).unwrap();
                        let core_dataset = file_output.new_dataset::<f64>().create("core", dataset.shape()).unwrap();
                        core_dataset.write(&core_array).unwrap();
                    }
                    Err(_) => {
                        println!("Havent core dataset");
                    }
                }
                // ------------------------------------------------------
            } else {
                println!("for while need be two dimensional");
            }
        }
        Err(_) => {
            println!("Cant find file");
        }
    }
}
