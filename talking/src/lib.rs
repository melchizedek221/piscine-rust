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
    if text.trim().is_empty() {
        return "Just say something!";
    }

    let is_all_caps = text.chars().all(|c| c.is_uppercase());
    let is_question = text.ends_with('?');

    if is_all_caps || is_question {
        "Quiet, I am thinking!"
    } else if is_all_caps {
        "There is no need to yell, calm down!"
    } else if is_question {
        "Sure."
    } else {
        "Interesting"
    }
}

