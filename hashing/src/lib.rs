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

use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    list.iter().map(|&i| i as f64).sum::<f64>() / list.len() as f64
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut sorted_list = list.clone();
    sorted_list.sort();

    let len = sorted_list.len();
    if len == 0 {
        panic!("Empty list!");
    }
    if len % 2 == 0 {
        let mid = len / 2;
        (sorted_list[mid - 1] + sorted_list[mid]) / 2
    } else {
        sorted_list[len / 2]
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut freq_map = HashMap::new();

    for &num in list {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    let mut mode_value = 0;
    let mut max_freq = 0;
    for (num, &freq) in &freq_map {
        if freq > max_freq {
            max_freq = freq;
            mode_value = *num;
        }
    }

    mode_value
}

// fn main() {
// 	println!("Hello, world!");
// 	let v = vec![4, 7, 5, 2, 5, 1, 3];
// 	println!("mean {}", mean(&v));
// 	println!("median {}", median(&v));
// 	println!("mode {}", mode(&v));
// }

// And its output;

// $ cargo run
// mean 3.857142857142857
// median 4
// mode 5
// $