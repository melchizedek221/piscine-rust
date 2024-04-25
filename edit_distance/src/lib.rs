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

use std::cmp::min;
pub fn edit_distance(source: &str, target: &str) -> usize {
    if source.len() == 0 {
        target.len() 
    }
    
    if target.len() == 0 { 
        source.len() 
    }
    
    if target.chars().nth(0).unwrap() == source.chars().nth(0).unwrap() {
        edit_distance(&source[1..], &target[1..])
    }
 
    1 + min(
        v1: min(
            v1: edit_distance(&source[1..], target),
            v2: edit_distance(source, &target[1..])
        ),
        v2: edit_distance(&source[1..], &target[1..])

    )
}