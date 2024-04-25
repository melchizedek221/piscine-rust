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

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}

pub fn sum(a: &[i32]) -> i32 {
    a.iter().sum()
}

// fn main() {
// 	let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
// 	let a1: Vec<i32> = (1..11).collect(); //missing info here
// 	let b = [32; 10]; //missing info here

// 	println!("The Sum of the elements in {:?} = {}", a, sum(&a));//missing info here
// 	println!("The Sum of the elements in {:?} = {}", a1, sum(&a1));//missing info here
// 	println!("The Sum of the elements in {:?} = {}", b, sum(&b));//missing info here
// 	println!(
// 		"Array size {} with only 10's in it {:?}",
// 		thirtytwo_tens().len(),
// 		thirtytwo_tens()
// 	);
// }