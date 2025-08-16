#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Self; 

    fn next(&mut self) -> Option<Self::Item> {
        if self.v <= 1 {
            return None;
        }
        let current = *self;  
        if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = 3 * self.v + 1;
        }
        Some(current)
    }
}

impl Collatz {
	pub fn new(n: u64) -> Self {
        Self{v : n}
    }
}

pub fn collatz(n: u64) -> usize {
    // Collatz::new(n).count()
    
    /*  can use also  */
      let mut a = 0;
    let mut b = n ;
    if b == 0 {
        return 0 ;
    }
    while b != 1 {
        if b%2 == 0 {
            b/=2 ;
        } else {
            b = 3*b +1 ;
        }
        // println!("{}", b);
        a += 1;
    }
    a
}