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
    if source.is_empty() {
        return target.len();
    }

    if target.is_empty() {
        return source.len();
    }

    if source.chars().next().unwrap() == target.chars().next().unwrap() {
        return edit_distance(&source[1..], &target[1..]);
    }

    1 + min(
        min(
            edit_distance(&source[1..], target),
            edit_distance(source, &target[1..]),
        ),
        edit_distance(&source[1..], &target[1..]),
    )
}
