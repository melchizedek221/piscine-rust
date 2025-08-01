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

use std::ops::Add;
pub fn add_curry<T>(a: T) -> impl Fn(T) -> T
where T: Add<Output=T> + Copy {
    move |b: T| a + b
}
pub fn twice<T>(f: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |b: T| f(f(b))
}