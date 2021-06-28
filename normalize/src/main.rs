use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    
    println!("{}", args.pattern);

    let _file = hdf5::File::open(args.path).unwrap();

    //for line in content.lines() {
    //    if line.contains(&args.pattern) {
    //        println!("{}", line);
    //    }
    //}
}
