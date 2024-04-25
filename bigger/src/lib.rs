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

use std::collections::HashMap;

// pub fn bigger(h: HashMap<&str, i32>) -> i32 {
//     let mut max_positive = i32::MIN;
//     println!("{:?}", h.values());
//     for &value in h.values() {
//         if value > 0 && value > max_positive {
//             max_positive = value;
//         }
//     }
//     max_positive
// }

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    
    *h.values().max().unwrap()
}

// fn main() {

//     let mut hash = HashMap::new();
//     hash.insert("Daniel", 122);
//     hash.insert("Ashley", 333);
//     hash.insert("Katie", 334);
//     hash.insert("Robert", 14);

//     println!("The biggest of the elements in the HashMap is {}", bigger(hash));
// }