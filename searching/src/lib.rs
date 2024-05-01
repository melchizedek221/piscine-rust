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

pub fn search(array: &[i32], key: i32) -> Option<usize> {
    array.iter().position(|&x| x == key)
}

// fn main() {
//     let ar = [1, 3, 4, 6, 8, 9, 11];
//     let f = search(&ar, 6);
//     println!(
//         "the element 6 is in the position {:?} in the array {:?}",
//         f, ar
//     );
// }