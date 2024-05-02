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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        match (self.r, self.g, self.b, self.a) {
            (r, g, _, _) if self.r == first && self.g == second => { self.r = second; self.g = first; }
            (r, _, b, _) if self.r == first && self.b == second => { self.r = second; self.b = first; }
            (r, _, _, a) if self.r == first && self.a == second => { self.r = second; self.a = first; }
            (_, g, r, _) if self.g == first && self.r == second => { self.g = second; self.r = first; }
            (_, g, _, b) if self.g == first && self.b == second => { self.g = second; self.b = first; }
            (_, g, _, a) if self.g == first && self.a == second => { self.g = second; self.a = first; }
            (_, _, b, r) if self.b == first && self.r == second => { self.b = second; self.r = first; }
            (_, _, b, g) if self.b == first && self.g == second => { self.b = second; self.g = first; }
            (_, _, b, a) if self.b == first && self.a == second => { self.b = second; self.a = first; }
            (_, _, _, r) if self.a == first && self.r == second => { self.a = second; self.r = first; }
            (_, _, _, g) if self.a == first && self.g == second => { self.a = second; self.g = first; }
            (_, _, _, b) if self.a == first && self.b == second => { self.a = second; self.b = first; }
            _ => {}
        }
        self
    }
}
