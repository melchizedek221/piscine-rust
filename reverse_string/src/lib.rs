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

pub fn rev_str(input: &str) -> String {
    let reversed_chars = input.chars().rev();
    let reversed_string: String = reversed_chars.collect();
    reversed_string
}

// fn main() {
//     println!("{}", rev_str("Hello, world!"));
//     println!("{}", rev_str("Hello, my name is Roman"));
//     println!("{}", rev_str("I have a nice car!"));
//     println!("{}", rev_str("How old are You"));
//     println!("{}", rev_str("ex: this is an example água"));
// }