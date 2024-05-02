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
            (r, g, _, _) if r == first && g == second => { self.r = second; self.g = first; }
            (r, _, b, _) if r == first && b == second => { self.r = second; self.b = first; }
            (r, _, _, a) if r == first && a == second => { self.r = second; self.a = first; }
            (_, g, r, _) if g == first && r == second => { self.g = second; self.r = first; }
            (_, g, _, b) if g == first && b == second => { self.g = second; self.b = first; }
            (_, g, _, a) if g == first && a == second => { self.g = second; self.a = first; }
            (_, _, b, r) if b == first && r == second => { self.b = second; self.r = first; }
            (_, _, b, g) if b == first && g == second => { self.b = second; self.g = first; }
            (_, _, b, a) if b == first && a == second => { self.b = second; self.a = first; }
            (_, _, _, r) if self.a == first && r == second => { self.a = second; self.r = first; }
            (_, _, _, g) if self.a == first && g == second => { self.a = second; self.g = first; }
            (_, _, _, b) if self.a == first && b == second => { self.a = second; self.b = first; }
            _ => {}
        }
        self
    }
}
