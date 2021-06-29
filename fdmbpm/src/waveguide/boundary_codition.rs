use crate::{fp::{matrix::MatrixView}, waveguide};
use num::complex::Complex;

pub enum Side {
    Left,
    Right
}

pub fn dirichlet(_: Side, _: &MatrixView<waveguide::Phasor, 1usize>) -> waveguide::Phasor {
    return *waveguide::zero();
}

pub fn transparent(s: Side, es: &MatrixView<waveguide::Phasor, 1usize>) -> waveguide::Phasor {
    match s {
        Side::Left => {
            let mut es_it = es.iter();
            let x0 = es_it.next().unwrap();
            let x1 = es_it.next().unwrap();

            let eta = x0/x1;
            
            let eta = if eta.re < 0.0 || eta.re.is_nan() {
                Complex::new(0.0, eta.im)
            } else {
                eta   
            };

            let eta = if eta.im.is_nan() {
                Complex::new(eta.re, 0.0)    
            } else {
                eta
            };

            eta
        },
        Side::Right => {
            let mut es_it = es.iter();
            let xn = es_it.next_back().unwrap();
            let xn_less_one = es_it.next_back().unwrap();

            let eta = (xn*1000000.0)/(xn_less_one*1000000.0);
            
            let eta = if eta.re < 0.0 || eta.re.is_nan() {
                Complex::new(0.0, eta.im)
            } else {
                eta   
            };

            let eta = if eta.im.is_nan() {
                Complex::new(eta.re, 0.0)    
            } else {
                eta
            };

            eta
        }
    }
}