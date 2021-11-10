use crate::functional_types;
use functional_types::list;
use super::*;
use types::*;
use lin_alg::{DiagonalMatrix, diagonal_matrix};
use grid::Grid;
use cores::Core;
use boundary_codition::Side;
use beam::Gaussian;

mod bidimensional;
mod tridimensional;

pub struct Slab<const D: usize, const N: usize> {
	grid: Grid<D>,
	core: Box<dyn Core<D>>, 
	beam: Gaussian<N>, 
	boundary_codition: fn(s: Side, es: &Vec<Phasor>)-> Phasor
}

pub fn new<const D: usize, const N: usize>(grid: Grid<D>, core: Box<dyn Core<D>>, beam: Gaussian<N>, boundary_codition: fn(s: Side, es: &Vec<Phasor>)-> Phasor) -> Slab<D,N> {
	match (D,N) {
		(2,1) | (3,2) => {
			Slab {grid, core, beam, boundary_codition }
		},
		_ => {
			panic!("Dimensões de core e feixe não estão consistentes")
		}
	}
}

impl<const D: usize, const N: usize> Slab<D,N> {
	
	fn get_es(&self, matrix: DiagonalMatrix, d: Vec<Phasor>) -> Vec<Phasor> {
		self.insert_boundary_values(lin_alg::thomas::try_solve(matrix, d))
	}
	
	fn insert_boundary_values(&self, es: Vec<Phasor>) -> Vec<Phasor> {
		let head = vec![{
			let es_head = functional_types::head_or_default(es.iter(), phasor::one());
			es_head* (self.boundary_codition)(Side::Left, &es)
		}];
		let last = vec![{
			let es_last = functional_types::last_or_default(es.iter(), phasor::one());
			es_last* (self.boundary_codition)(Side::Right, &es)
		}];
		
		list::concat(list::concat(head, es),last)
	}

	fn equation_to_diagonal_matrix(&self, s: Vec<Phasor>, last_es: &Vec<Phasor>) -> DiagonalMatrix {
	
		let len = s.len();
		
		let mut below_diag = vec![-(*phasor::one()); len];
		below_diag[0] = *phasor::zero();
		
		let diag = self.get_diagonal(s, last_es);
	
		let mut above_diag = vec![-(*phasor::one()); len];
		above_diag[len - 1] = *phasor::zero();
	
		diagonal_matrix(below_diag, diag, above_diag)
	
	}
	
	fn get_diagonal(&self, mut s: Vec<Phasor>, last_es: &Vec<Phasor>) -> Vec<Phasor> {
		let left_boundary = (self.boundary_codition)(Side::Left, last_es);
		let right_boundary = (self.boundary_codition)(Side::Right, last_es);
		
		s[0] = s[0] - left_boundary;
		let len = s.len();
		s[len - 1] = s[len - 1] - right_boundary;
	
		s
	}
}

// Colocar nome das equações no okamoto
trait SlabParamtersFormulas<const D: usize>  {
	fn s(&self, position: [usize;D], zdelta: f64, delta: f64, k: f64, alpha: f64) -> Phasor {
		let (guiding_space, free_space, loss) = self.slab_formulas(position, zdelta, delta, k, alpha);
		Complex::new(2.0 - guiding_space, free_space + loss)
	}
	
	fn q(&self, position: [usize;D], zdelta: f64, delta: f64, k: f64, alpha: f64) -> Phasor {
		let (guiding_space, free_space, loss) = self.slab_formulas(position, zdelta, delta, k, alpha);
		Complex::new(-2.0 + guiding_space, free_space - loss)
	}

	fn slab_formulas(&self, position: [usize;D], zdelta: f64, delta: f64, k: f64, alpha: f64) -> (f64, f64, f64) {
		let guiding_space = self.guiding_space(position, delta, k);
		let free_space = self.free_space(zdelta, delta, k);
		let loss = self.loss(delta, k, alpha);
	
		(guiding_space, free_space, loss)
	}

	fn guiding_space(&self, position: [usize;D], delta: f64, k: f64) -> f64;
	
	fn free_space(&self, zdelta: f64, delta: f64, k: f64) -> f64;
	
	fn loss(&self, delta: f64, k: f64, alpha: f64) -> f64;
}

fn get_const_terms(es: &Vec<Phasor>, qs: Vec<Phasor>) -> Vec<Phasor> {
	functional_types::middle(qs.iter()).enumerate().map(
		// okamoto 7.97
		|(i, q)| es[i]+q*es[i+1]+es[i+2]
	).collect()
}