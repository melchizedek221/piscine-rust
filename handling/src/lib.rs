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
// use std::fs::File;

pub fn open_or_create(file: &str, content: &str) {
    match std::fs::write(file, content) {
        Ok(_) => println!("Created"),
        Err(err) => panic!("Error {}", err),
    }
}
