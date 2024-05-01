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



pub fn is_pangram(s: &str) -> bool {
    const ALPHABET = "abcdefghijklmnopqrstuvwxyz";
    ALPHABET.chars().all(|ch| s.to_lowercase().contains(ch))
}