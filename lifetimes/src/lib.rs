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

#[derive(Debug, Clone)]
pub struct Person<'a> {
    pub name: &'a str,
    pub age: u8,
}

impl<'a> Person<'a> {
    pub fn new(name: &'a str) -> Person<'a> {
        Person {
            name: name,
            age: 0,
        }
    }
}
