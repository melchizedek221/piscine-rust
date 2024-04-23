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

fn nbr_function(x: i32) -> (f64, f64) {
    let x_f64: f64 = x as f64;

    let exp_x = x_f64.exp();
    let ln_x = x_f64.ln();

    (exp_x, ln_x)
}

pub fn str_function(a: String) -> (String, String) {
    let exp_values: Vec<String> = a.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap() as f64)
        .map(|f| f.exp().to_string())
        .collect();

    (a, exp_values.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ln_values: Vec<f64> = b.iter()
        .map(|&val| if val == 0 {
            f64::NEG_INFINITY
        } else {
            (val.abs() as f64).ln()
        })
        .collect();

    (b, ln_values)
}

// fn main() {
//     let a: i32 = 0;
//     let b = String::from("1 2 4 5 6");
//     let c = vec![1, 2, 4, 5];

//     println!("{:?}", nbr_function(a));
//     println!("{:?}", str_function(b));
//     println!("{:?}", vec_function(c));
// }