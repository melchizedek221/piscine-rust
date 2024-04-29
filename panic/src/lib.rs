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

pub fn open_file(s: &str) -> File {
    match File::open(s){
        Ok(f) => f,
        Err(_) => panic!("File not found")
    }

}