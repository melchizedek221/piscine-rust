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


pub fn first_subword(mut s: String) -> String {
    for (idx, c) in s.char_indices() {
        if c == '_' || c.is_ascii_uppercase() && idx != 0 {
            return s[..idx].to_string();
        }
    }
    s
}