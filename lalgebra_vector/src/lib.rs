use lalgebra_scalar::Scalar;
use std::ops::Add;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> 
where 
    T: Add<Output = T> + Clone,
{
    type Output = Option<Self>;
    
    fn add(self, other: Self) -> Option<Self> {
        if self.len() != other.len() {
            return None;
        }

        let mut a = Vec::new();
        let j = min(self.len(), other.len());
        
        for i in 0..j {
            a.push(self.0[i].clone() + other.0[i].clone());
        }
        
        Some(Vector(a))
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }
    
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn dot(&self, other: &Self) -> Option<T> 
        where
        T: Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Copy,
    {
        if self.len() != other.len() {
            return None;
        }
        
        let mut sum = self.0[0] - self.0[0]; 
        for i in 0..self.len() {
            sum = sum + (self.0[i] * other.0[i]);
        }
        Some(sum)
    }
}

pub fn min(a: usize, b: usize) -> usize {
    if a < b {
        return a;
    }
    b
}