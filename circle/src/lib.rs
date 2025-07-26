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

    pub fn intersect(&self, other: Circle) -> bool {
        self.center.distance(other.center) <= maxx(self.radius, other.radius)
    }
}

pub fn maxx(a: f64,b :f64)-> f64 {
    /*to get the largest radius and compare it to the distance between the two circles */

    if a >= b {
        return a;
    }
    b
}

/*

(x-x0)2 + (y-y0)2 = r2
(x-x1)2 + (y-y1)2 = rr2

2x(x1-x0) +x12 + 2y(y1-y0) + y12 = r2-rr2

x(x1-x0) + y(y1-y0) = (r2-rr2 - x12 - y12)/2

*/
