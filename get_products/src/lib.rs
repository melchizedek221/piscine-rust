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

pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let product: usize = arr.iter().product();
    arr.into_iter().map(|n| product / n).collect()
}