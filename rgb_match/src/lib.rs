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
        match (first, second) {
            (0, 1) => { let temp = self.r; self.r = self.g; self.g = temp; }
            (0, 2) => { let temp = self.r; self.r = self.b; self.b = temp; }
            (0, 3) => { let temp = self.r; self.r = self.a; self.a = temp; }
            (1, 2) => { let temp = self.g; self.g = self.b; self.b = temp; }
            (1, 3) => { let temp = self.g; self.g = self.a; self.a = temp; }
            (2, 3) => { let temp = self.b; self.b = self.a; self.a = temp; }
            _ => {}
        }
        self
    }
}