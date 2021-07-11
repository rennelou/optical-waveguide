use crate::fp::matrix::Idx;

use super::*;
use cores::Core;
use Phasor;
use fp::list;
use fp::Matrix;

pub fn run(core: &impl Core<3>, k: f64, alpha: f64, e_input: Matrix<Phasor>, boundary_codition: fn(s: Side, es: &MatrixView<Phasor, 1usize>) -> Phasor) -> EletricField {
	let grid_steps = core.get_deltas().to_vec();
	let [zdepht, ydepht, xdepht] = core.get_shape().clone();

	let (s_x, s_y, q_x, q_y) = get_initialized_params_3d(core, k, alpha);

	let es = (1usize..zdepht).fold(
		vec![e_input], 
		|result, z| {
			
			let d_list = (1..xdepht-1).map(|x| {
				let last_es= fp::last(result.iter()).unwrap().view(&[Idx::Free, Idx::Value(x)]);
				let last_qy = q_y.view(&[Idx::Value(z-1), Idx::Free, Idx::Value(x)]);
				
				// multiplicar d pelo fator para 3 dimensões
				get_ds(&last_es, last_qy)
			}).collect();
			let transposed_d_plane = matrix::zip(d_list);

			let es_list = (1..ydepht-1).map(|y| {
				let last_es= fp::last(result.iter()).unwrap().view(&[ Idx::Value(y),Idx::Free]);

				let sx_list = s_x.view(&[Idx::Value(z-1), Idx::Value(y), Idx::Free]);
				let d_list = transposed_d_plane.view(&[Idx::Free, Idx::Value(y-1)]);

				get_es(sx_list, d_list, last_es, boundary_codition)
			}).collect();
			let es_intermediate = matrix::zip(es_list);
			
//----------------------- segunda parte -----------------------------------------------

			let h_list = (1..ydepht-1).map(|y|{
				let last_es = es_intermediate.view(&[Idx::Value(y-1), Idx::Free]);
				let last_qx = q_x.view(&[Idx::Value(z-1), Idx::Value(y), Idx::Free]);

				// multiplicar d pelo fator para 3 dimensões
				get_ds(&last_es, last_qx)
			}).collect();
			let h_plane = matrix::zip(h_list);
			
			let es_list = (1..xdepht-1).map(|x|{
				let last_es= fp::last(result.iter()).unwrap().view(&[Idx::Free,Idx::Value(x)]);

				let sy_list = s_y.view(&[Idx::Value(z-1), Idx::Free, Idx::Value(x)]);
				let h_list = h_plane.view(&[Idx::Free, Idx::Value(x-1)]);

				get_es(sy_list, h_list, last_es, boundary_codition)
			}).collect();
			let es_transposed = matrix::zip(es_list);
			
			let es_list = (0..ydepht).map(|y|{
				let es_to_insert_boundary_x = es_transposed.view::<1>(&[Idx::Free, Idx::Value(y)]).iter().cloned().collect();
				insert_boundary_values(es_to_insert_boundary_x, boundary_codition)
			}).collect();
			let es = matrix::zip(es_list);

			list::append(result, es)
		}
	);

	let values = matrix::zip(es);
	return EletricField { values, grid_steps };
}

pub fn get_initialized_params_3d(core: &impl Core<3>, k: f64, alpha: f64) 
-> (Matrix<Phasor>, Matrix<Phasor>, Matrix<Phasor>, Matrix<Phasor>) {
	let shape = core.get_shape().clone();
	let [zdelta, ydelta, xdelta] = core.get_deltas().clone();
	let n0 = core.get_n0();

    let guiding_space = |position: Vec<_>, delta: f64| ((k.powf(2.0)*delta.powf(2.0))/2.0)*(core.get_half_n(position.as_slice(), n0).powf(2.0)-n0.powf(2.0));
    let free_space = |delta: f64| 4.0*k*n0*delta.powf(2.0)/zdelta;
    let loss = |delta: f64| 2.0*k*n0*delta.powf(2.0)*alpha;
    
	let get_params = |delta: f64| -> (Vec<Phasor>,Vec<Phasor>) {
		(0..shape.iter().product()).map(|id| -> (Complex<f64>, Complex<f64>) {
        	let position = matrix::id_to_position(id, &shape);
        	(
				// okamoto 7.126/7.127
				Complex::new(2.0 - guiding_space(position.clone(), delta), free_space(delta) + loss(delta)),
				// okamoto 7.128/7.129
				Complex::new(-2.0 + guiding_space(position.clone(), delta), free_space(delta) - loss(delta))
			)

		}).unzip()
	};

	let (s_x, q_x) = get_params(xdelta);
	let (s_y, q_y) = get_params(ydelta);

	let shape_vec = shape.to_vec();
    let s_x = matrix::new(s_x, &shape_vec);
    let s_y = matrix::new(s_y, &shape_vec);
    let q_x = matrix::new(q_x, &shape_vec);
    let q_y = matrix::new(q_y, &shape_vec);
    
    (s_x, s_y, q_x, q_y)
}

#[cfg(test)]
mod tests {

	use ndarray::Array;
	use crate::{waves};

	use super::*;
	use std::{error::Error, f64::consts::PI};

	#[test]
	fn slab3d() -> Result<(), Box<dyn Error>> {
		let k0 = (2.0*PI)/1.15;

		let xdepht = 100usize;
		let ydepht = 100usize;

    	let dx = 10.0;
    	let xdelta = dx/(xdepht as f64);

		let dy = 10.0;
		let ydelta = dy/(ydepht as f64);
		
		let dz = 200.0;
    	let zdelta = 0.5;
    	
    	let position_x = dx/2.0;
		let position_y = dy/2.0;
    	let width = 8.0;
		
		let shape = [ydepht, xdepht];
		let deltas = [ydelta, xdelta];
		let center = [position_y, position_x];

    	let n0 = 3.377;
    	let n = 3.38;

    	let core = cores::rectilinear::new_3d(dx, xdelta, dy, ydelta, dz, zdelta, n, n0, position_x, width);
		
    	let w = 2.0;
    	let gaussian = waves::gaussian(&shape, &deltas, &center, 1.0, w);

    	let e = fdmbpm::slab3d::run(&core, k0, 0.0, gaussian, boundary_codition::transparent);
		// para gerar seria so exportar e -- export::hdf5("slab3d.h5", &e, &core);

		let intensity = e.get_intensity();
    	let array = Array::from_shape_vec(e.shape().clone(), intensity)?;

    	let file = hdf5::File::open("slab3d.h5")?;
		let values = file.dataset("intensity")?;

		assert_eq!(values.read_dyn::<f64>()?, array);

		Ok(())
   	}
}