use optical_waveguide::simulator::fdmbpm;
use structopt::StructOpt;
use std::fs;

#[derive(StructOpt)]
struct WaveguideArgs {
    #[structopt(parse(from_os_str))]
    input_file: std::path::PathBuf,
    output_name: String
}

pub fn main() {
    let args = WaveguideArgs::from_args();

    if let Ok(input) = fs::read_to_string(args.input_file) {
        fdmbpm::run(&input, &args.output_name);
    } else {
        panic!("cant open the input file")
    }
}