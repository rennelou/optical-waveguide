use super::*;
use super::boundary_codition::Side;
use crate::fp::{self, matrix};
use crate::fp::list;
use crate::lin_alg::{self, DiagonalMatrix, diagonal_matrix};

pub mod slab2d;
pub mod slab3d;

fn get_ds(es: &Vec<Phasor>, qs: Vec<Phasor>) -> Vec<Phasor> {
	qs.iter().enumerate().map(
		// okamoto 7.97
		|(i, q)| es[i]+q*es[i+1]+es[i+2]
	).collect()
}

fn equation_to_diagonal_matrix(s: Vec<Phasor>, last_es: &Vec<Phasor>, boundary_codition: fn(s: Side, es: &Vec<Phasor>)->Phasor) -> DiagonalMatrix {
	
	let len = s.len();
	
	let mut below_diag = vec![-(*one()); len];
	below_diag[0] = *zero();
	
	let diag = get_diagonal(s, last_es, boundary_codition);

	let mut above_diag = vec![-(*one()); len];
	above_diag[len - 1] = *zero();

	diagonal_matrix(below_diag, diag, above_diag)

}

fn get_diagonal(mut s: Vec<Phasor>, last_es: &Vec<Phasor>, boundary_codition: fn(s: Side, es: &Vec<Phasor>)->Phasor) -> Vec<Phasor> {
	
	let left_boundary = boundary_codition(Side::Left, last_es);
	let right_boundary = boundary_codition(Side::Right, last_es);
	
	s[0] = s[0] - left_boundary;
	let len = s.len();
	s[len - 1] = s[len - 1] - right_boundary;

	s
}

fn get_es(matrix: DiagonalMatrix, d: Vec<Phasor>, boundary_codition: fn(s: Side, es: &Vec<Phasor>)->Phasor) -> Matrix<Phasor> {
	insert_boundary_values(
		lin_alg::thomas::try_solve(matrix, d),
		boundary_codition
	)
}

fn insert_boundary_values(es: Vec<Phasor>, boundary_codition: fn(s: Side, es: &Vec<Phasor>) -> Phasor) -> Matrix<Phasor> {

	let head = vec![{
		let es_head = fp::head_or_default(es.iter(), one());
		es_head*boundary_codition(Side::Left, &es)
	}];
	let last = vec![{
		let es_last = fp::last_or_default(es.iter(), one());
		es_last*boundary_codition(Side::Right, &es)
	}];
	
	let values = list::concat(list::concat(head, es),last);

	matrix::new(values)
}