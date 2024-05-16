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

fn first_fifty_even_square() -> Vec<i32> {
    (2..).step_by(2).map(|x| x * x).take(50).collect()
}
