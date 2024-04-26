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

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut char_count_s1: HashMap<char, usize> = HashMap::new();
    let mut char_count_s2: HashMap<char, usize> = HashMap::new();

    for ch in s1.chars() {
        *char_count_s1.entry(ch).or_insert(0) += 1;
    }

    for ch in s2.chars() {
        *char_count_s2.entry(ch).or_insert(0) += 1;
    }

    char_count_s1 == char_count_s2
}