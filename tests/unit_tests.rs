#[cfg(test)]
mod tests {
    use rust_fdmbpm::waveguide;
    use num::complex::Complex;

    #[test]
    fn assert_abcs_sizes() {
        for i in 1..10 {
            let _ = get_waveguide_mock(100.0, i as f64, 2.0, 1.0, 1.0/1550.0, 3.4757, 1.0, 0.2, Complex::new(0.0, 0.0), Complex::new(0.0, 0.0));
        }
    }

    fn get_waveguide_mock(dx: f64, xdelta: f64, dz: f64, zdelta: f64,
    	k: f64, n: f64, n0: f64, alpha: f64, kleft: Complex<f64>, kright: Complex<f64>) -> waveguide::Slab {
    
        return waveguide::Slab::new(dx, xdelta, dz, zdelta, k, n, n0, alpha, kleft, kright);
    }
}