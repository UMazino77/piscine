use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla,
	I,
	V,
	X,
	L,
	C,
	D,
	M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
	fn from(value: u32) -> Self {
		match value {
			0 => Nulla,
			1 => I,
			5 => V,
			10 => X,
			50 => L,
			100 => C,
			500 => D,
			1000 => M,
			_ => panic!("Error"),
		}
	}
}

impl From<u32> for RomanNumber {
	fn from(value: u32) -> Self {
		let mut a = Vec::new();
		let mut c = value;

		if c == 0 {
			a.push(RomanDigit::Nulla);
			return RomanNumber(a);
		}

		while c > 0 {
			if c >= 1000 {
				a.push(M);
				c -= 1000;
			} else if c >= 900 {
				a.push(C);
				a.push(M);
				c -= 900;
			} else if c >= 500 {
				a.push(D);
				c -= 500;
			} else if c >= 400 {
				a.push(C);
				a.push(D);
				c -= 400;
			} else if c >= 100 {
				a.push(C);
				c -= 100;
			} else if c >= 90 {
				a.push(X);
				a.push(C);
				c -= 90;
			} else if c >= 50 {
				a.push(L);
				c -= 50;
			} else if c >= 40 {
				a.push(X);
				a.push(L);
				c -= 40;
			} else if c >= 10 {
				a.push(X);
				c -= 10;
			} else if c == 9 {
				a.push(I);
				a.push(X);
				c -= 9;
			} else if c >= 5 {
				a.push(V);
				c -= 5;
			} else if c == 4 {
				a.push(I);
				a.push(V);
				c -= 4;
			} else if c >= 1 {
				a.push(I);
				c -= 1;
			}
		}

		RomanNumber(a)
	}
}