pub mod simulator;
pub mod functional_types;
pub mod tools;

use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "optical_waveguide")]
fn python_wrapper(_py: Python, m: &PyModule) -> PyResult<()> {
    
    #[pyfn(m)]
    fn run(serialized: &str, output_name: &str) {
        simulator::fdmbpm::run(serialized, output_name);
    }
    
    Ok(())
}