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
pub fn delete_prefix(prefix: &str, s: &str) -> Option<&str> {
    if s.starts_with(prefix) {
        return Some(&s[prefix.len()..]);
    }
    None
}