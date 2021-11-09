pub mod simulator;
pub mod tools;

use pyo3::prelude::*;

// A Python module implemented in Rust. The name of this function must match
// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
// import the module.
#[pymodule]
#[pyo3(name = "optical_waveguide")]
fn optical_waveguide(_py: Python, m: &PyModule) -> PyResult<()> {
    
    #[pyfn(m)]
    fn run(serialized: &str, output_name: &str) {
        simulator::fdmbpm::run(serialized, output_name);
    }
    
    Ok(())
}