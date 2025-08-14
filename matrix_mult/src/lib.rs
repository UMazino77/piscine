// use matrix::*;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;


use lalgebra_scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T> + std::clone::Clone> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix(vec![])
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut vec = Vec::with_capacity(row);
        for _ in 0..row {
            vec.push(vec![T::zero(); col]);
        }
        Matrix(vec)
	}

	pub fn identity(n: usize) -> Matrix<T> {
        let mut vec = Vec::with_capacity(n);
        for i in 0..n {
            let mut row = vec![T::zero(); n];
            row[i] = T::one();
            vec.push(row);
        }
        Matrix(vec)
	}
	pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            return 0;
        } 
        self.0[0].len()
	}

	pub fn number_of_rows(&self) -> usize {
        self.0.len()
	}

	pub fn row(&self, n: usize) -> Vec<T> {
		if n >= self.number_of_rows() {
			return Vec::new();
		}
		self.0[n].clone()
	}

	pub fn col(&self, n: usize) -> Vec<T> {
		if n >= self.number_of_cols() {
			return Vec::new();
		}
		let mut a = vec![];
		self.0.iter().for_each(|x| a.push(x[n].clone()));
		
		a
	}
}

impl<T> Mul for Matrix<T> where T: Scalar<Item = T> + Clone,T: Add<Output = T> + Mul<Output = T>{
	type Output = Option<Matrix<T>>;

	fn mul(self, other: Matrix<T>) -> Option<Matrix<T>> {
		if self.number_of_cols() != other.number_of_rows() {
			return None;
		}
		let a = self.number_of_rows();
		let b = other.number_of_cols();
		let mut result = Matrix::zero(a, b);

		for i in 0..a {
			for j in 0..b {
				let mut sum = T::zero();
				for k in 0..self.number_of_cols() {
					sum = sum + (self.0[i][k].clone() * other.0[k][j].clone());
				}
				result.0[i][j] = sum;
			}
		}
		Some(result)
	}
}