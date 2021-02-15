#[cfg(test)]
mod tests {
    use rust_fdmbpm::waveguide::slab;
    use num::complex::Complex;

    #[test]
   	fn assert_fdmbpm() {
   	    let w = slab::mock::get_waveguide_mock(10.0, 2.0, 10.0, 5.0, 1.0/1550.0, 3.4757, 1.0, 0.2, Complex::new(1.0, 0.0), Complex::new(1.0, 0.0));
		let got = w.fdmbpm(slab::mock::get_ones(5));
		
        println!("{:?}", got[1]);
   	}
}