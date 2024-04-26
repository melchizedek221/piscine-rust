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

pub fn bubble_sort(vec: &mut Vec<i32>) {
    // let mut is = false;
    // while !is{
    //     is = true;
    //     for i in 0..vec.len() - 1{
    //         if vec[i] > vec [i + 1] {
    //             vec.swap(i, i+1);
    //             is = false;
    //         }
    //     }
    // }
    
    vec.sort()
}