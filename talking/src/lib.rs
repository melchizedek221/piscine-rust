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

pub fn talking(text: &str) -> &str {
    if text.is_empty() {
        "Just say something!"
    } else if text.is_ascii_uppercase() {
        if text.contains('?') {
            "Quiet, I am thinking!"
        } else {
            "There is no need to yell, calm down!"
        }
    } else {
        "Sure."
    }
}
