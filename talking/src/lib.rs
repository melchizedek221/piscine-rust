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
    match text.trim() {
        "" => "Just say something!",
        _ => {
            let has_alphabetic = text.chars().any(|c| c.is_alphabetic());
            let is_all_caps = has_alphabetic && text.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
            let is_question = text.trim_end().ends_with('?');

            match (is_all_caps, is_question) {
                (true, _) if is_question => "Quiet, I am thinking!",
                (true, _) => "There is no need to yell, calm down!",
                (false, true) => "Sure.",
                _ => "Interesting",
            }
        }
    }
}
