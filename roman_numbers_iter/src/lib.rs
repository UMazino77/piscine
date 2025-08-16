#[derive(Debug, Clone, PartialEq, Eq)]

pub struct RomanNumber {
    value: Vec<RomanNumeral>,
}


#[derive(Debug, Clone, Eq, PartialEq, Copy)]

pub enum RomanNumeral {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

impl RomanNumber {
    pub fn from(value: u64) -> Self {
        let mut a = Vec::new();
        let mut b = value;

        while b > 0 {
            if b >= 1000 {
                a.push(RomanNumeral::M);
                b -= 1000;
            } else if b >= 500 {
                a.push(RomanNumeral::D);
                b -= 500;
            } else if b >= 100 {
                a.push(RomanNumeral::C);
                b -= 100;
            } else if b >= 50 {
                a.push(RomanNumeral::L);
                b -= 50;
            } else if b >= 10 {
                a.push(RomanNumeral::X);
                b -= 10;
            } else if b >= 5 {
                a.push(RomanNumeral::V);
                b -= 5;
            } else if b >= 1 {
                a.push(RomanNumeral::I);
                b -= 1;
            }
        }
        RomanNumber{ value: a}
    }
    pub fn to(&self, a : Vec<RomanNumeral>) -> u64 {
        let mut b = 0;
        for numeral in &self.value {
            b += *numeral as u64;
        }
        b
    }

}

impl Iterator for RomanNumber {
    type Item = RomanNumber;


     fn next(&mut self) -> Option<Self> {
        if self.value.is_empty() {
            return None;
        }
        Some(RomanNumber::from(self.to(self.value.clone())+1))
    }

}    