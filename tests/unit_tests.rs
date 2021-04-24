#[cfg(test)]
mod tests {
    use rust_fdmbpm::waveguide::core_waveguide;
	use rust_fdmbpm::waveguide::slab;

    #[test]
   	fn assert_fdmbpm() {
		let r = core_waveguide::rectilinear::new(10.0, 2.0, 10.0, 5.0, 3.4757, 1.0, 5.0, 2.0);
   	    let _w = slab::new(&r, 1.0/1550.0, 0.2);
		//let _ = slab::fdmbpm(&w, slab::mock::get_ones(5));
   	}
}