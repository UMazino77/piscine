#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       if &s[..s.len()-1] == "AB" {
        return Ok(Antigen::AB);
       } else if &s[..s.len()-1] == "A" {
        return Ok(Antigen::A);
       } else if &s[..s.len()-1] == "B" {
        return Ok(Antigen::B);
       } else if &s[..s.len()-1] == "O" {
        return Ok(Antigen::O);
       }
       Err("type doesn't exist")
    }
}

impl FromStr for RhFactor {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       if s.contains("+") {
        return Ok(RhFactor::Positive);
       } else if s.contains("-") {
        return Ok(RhFactor::Negative);
       }
       Err("type doesn't exist")
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            ord => ord,
        }
    }
}

impl FromStr for BloodType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antigen = Antigen::from_str(s)?;
        let rh_factor = RhFactor::from_str(s)?;
        Ok(BloodType { antigen, rh_factor })
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.rh_factor == RhFactor::Positive {
            write!(f, "{:?}+", self.antigen)
        } else {
            write!(f, "{:?}-", self.antigen)
        }
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        println!("{:?} can receive from  {:?}", self, other);
        
match self.rh_factor {
    RhFactor::Negative => {
        return other.rh_factor == RhFactor::Negative && (other.antigen == Antigen::O || other.antigen == self.antigen || self.antigen == Antigen::AB);},
    RhFactor::Positive => {
        return other.antigen == Antigen::O || other.antigen == self.antigen || self.antigen == Antigen::AB;}    
}
    }

    pub fn donors(&self) -> Vec<Self> {
        let st = "O+ A+ B+ AB+ O- A- B- AB-";
        let mut aa = Vec::new();
        for blood in st.split_whitespace() {
            if let Ok(a) = BloodType::from_str(blood) {
                if self.can_receive_from(&a) {
                    aa.push(a);
                }
            } else {
                panic!("type doesn't exist");
            }
        }
        aa.sort();
        aa
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let st = "O+ A+ B+ AB+ O- A- B- AB-";
        let mut aa = Vec::new();
        for blood in st.split_whitespace() {
            if let Ok(a) = BloodType::from_str(blood) {
                if a.can_receive_from(&self) {
                    println!("{:?} can receive from {:?}", a, self);
                    aa.push(a);
                }
            }
        }
        aa.sort();
        aa
    }
}
