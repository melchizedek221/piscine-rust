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

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub(i32, i32), pub(i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    let Matrix((x, y), (z, a)) = m;

    Matrix((x, z), (y, a))
}

