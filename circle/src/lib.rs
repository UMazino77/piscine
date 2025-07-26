use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64,pub f64);

impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        ((other.1-self.1).powf(2.0)+(other.0-self.0).powf(2.0)).sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center :Point,
	pub radius :f64
}

impl Circle {
    pub fn new(x: f64, y: f64, rad:f64) -> Self {
        Circle {
            center : Point(x,y),
            radius : rad
        }
	}
    pub fn diameter(&self) -> u32 {
        (2.0*self.radius) as u32
    }

    pub fn area(&self) -> f64 {
        2.0*self.radius*PI
    }

    // fn intersect(&self, other: &Circle) -> u32 {
    //     todo!()
    // }
}
