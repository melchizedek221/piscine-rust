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

// fn add_curry(x: i32) -> Box<dyn Fn(i32) -> i32> {
//     Box::new(move |y| x + y)
// }

use std::ops::Add;
pub fn add_curry<T>(a: T) -> impl Fn(T) -> T
where T: Add<Output=T> + Copy {
    move |b: T| a + b
}