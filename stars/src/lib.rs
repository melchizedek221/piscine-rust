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

pub fn stars(n: u32) -> String {
    let num_stars = 2u32.pow(n); 
    "*".repeat(num_stars as usize) 
}

// fn main() {
//     println!("{}", stars(1));
//     println!("{}", stars(4));
//     println!("{}", stars(5));
// }