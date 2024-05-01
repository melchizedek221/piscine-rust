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

pub fn number_logic(num: u32) -> bool {
    let digits = num.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let num_digits = digits.len() as u32;

    let mut sum = 0;
    for digit in &digits {
        sum += digit.pow(num_digits);
    }

    sum == num
}
