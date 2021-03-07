#[cfg(test)]
mod tests {
    use rust_fdmbpm::waveguide::refractive_index;
	use rust_fdmbpm::array;
	use rust_fdmbpm::waveguide::slab;
    use num::complex::Complex;

    #[test]
   	fn assert_fdmbpm() {
		let geometry = array::Array2d::new(10.0, 2.0, 10.0, 5.0);
		let r = refractive_index::optical_fiber::new(3.4757, 1.0, 4.5, 7.5);
   	    let w = slab::new(&geometry, 1.0/1550.0, r, 0.2, Complex::new(1.0, 0.0), Complex::new(1.0, 0.0));
		let _ = w.fdmbpm(slab::mock::get_ones(5));
   	}
}