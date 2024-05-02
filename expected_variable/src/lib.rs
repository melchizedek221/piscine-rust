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

use case::{CaseExt, Case};

//new edit distance
pub fn edit_distance(source: &str, target: &str) -> usize {
    let len_source = source.chars().count();
    let len_target = target.chars().count();

    let mut dp: Vec<Vec<usize>> = vec![vec![0 as usize; len_target + 1]; len_source + 1];

    for i in 1..=len_source {
        dp[i][0] = i;
    }

    for j in 1..=len_target {
        dp[0][j] = j;
    }

    let mut substitution_cost: usize;
    for j in 1..=len_target {
        for i in 1..=len_source {
            if source.chars().nth(i - 1).unwrap() == target.chars().nth(j - 1).unwrap() {
                substitution_cost = 0;
            } else {
                substitution_cost = 1;
            }

            dp[i][j] = std::cmp::min(
                dp[i - 1][j] + 1,
                std::cmp::min(dp[i][j - 1] + 1, dp[i - 1][j - 1] + substitution_cost),
            );
        }
    }

    dp[len_source][len_target]
}

pub fn expected_variable(target_str: &str, expected_str: &str) -> Option<String> {
    let target = target_str.to_lowercase();
    let expected = expected_str.to_lowercase();

    if !target.is_camel_lowercase() && !target.is_snake_lowercase() {
        return None;
    }

    let differ_chars = edit_distance(&target, &expected);
    let exp = 1.0 - differ_chars as f64 / expected.len() as f64;
    let exp = (exp * 100.0).round() as usize;

    if exp > 50 {
        Some(format!("{}%", exp))
    } else {
        None
    }
}
