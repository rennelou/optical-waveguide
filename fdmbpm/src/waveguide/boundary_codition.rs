use crate::waveguide;
use num::complex::Complex;

pub enum Side {
    Left,
    Right
}

pub fn dirichlet(_: Side, _: &Vec<waveguide::Phasor>) -> waveguide::Phasor {
    return *waveguide::zero();
}

pub fn transparent(s: Side, es: &Vec<waveguide::Phasor>) -> waveguide::Phasor {
    
    // forma mais simples que considera que a frente de onda é transversal ao eixo z. 
    // caso implemente semi vector ou full vector algoritmo é interessante ver os mais casos
    // do Hadley 1992 pra garantir que passos muito grandes em Z não cause problemas
    match s {
        Side::Left => {
            let mut es_it = es.iter();
            let x0 = es_it.next().unwrap();
            let x1 = es_it.next().unwrap();

            let eta = x0/x1;

            valid_eta(eta)
        },
        Side::Right => {
            let mut es_it = es.iter();
            let xn = es_it.next_back().unwrap();
            let xn_less_one = es_it.next_back().unwrap();

            let eta = (xn*1000000.0)/(xn_less_one*1000000.0);
            
            valid_eta(eta)
        }
    }
}

fn valid_eta(eta: Complex<f64>) -> Complex<f64> {
    
    let tmp = if eta.re < 0.0 || eta.re.is_nan() {
        Complex::new(0.0, eta.im)
    } else {
        eta   
    };

    let result = if tmp.im.is_nan() {
        Complex::new(tmp.re, 0.0)    
    } else {
        tmp
    };

    result
}