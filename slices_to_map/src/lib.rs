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

pub fn slices_to_map<'a, T: std::cmp::Eq + std::hash::Hash, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U> {
    let min_len = std::cmp::min(keys.len(), values.len());
    let key_slice = &keys[..min_len];
    let value_slice = &values[..min_len];
    key_slice.iter().zip(value_slice.iter()).collect::<HashMap<_, _>>()
}

