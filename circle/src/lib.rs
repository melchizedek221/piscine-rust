// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

#[derive(Debug)]
pub struct Circle {
	pub center: Point,
	pub radius: f64
}

impl Circle {
    // ...
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Self {
            center: Point::new(x, y),
            radius
        }
    }
    
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn intersect(&self, other: &Circle) -> bool {
        self.center.distance(&other.center) <= self.radius + other.radius
    }
}

#[derive(Debug)]
pub struct Point {
    // ...
    pub x: f64,
    pub y: f64
}

impl Point {
    // ...
    pub fn new(x: f64, y: f64) -> Self {
        Self{x, y}
    }
    
    pub fn distance(&self, other: &Point ) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}