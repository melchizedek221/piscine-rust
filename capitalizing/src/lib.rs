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

pub fn capitalize_first(input: &str) -> String {
    if let Some(first_char) = input.chars().next() {
        let cap = first_char.to_uppercase();
        let rest = &input[1..];
        format!("{}{}", cap, rest)
    }else{
        input.to_string()
    }
}

pub fn title_case(input: &str) -> String {
    input.split_whitespace()
        .map(|title| capitalize_first(title))
        .collect::<Vec<_>>()    
        // .collect()
        .join(" ")

}

pub fn change_case(input: &str) -> String {
    input.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c
            }
        })
        .collect()
}

// fn main() {
//     println!("{}", capitalize_first("joe is missing"));
//     println!("{}", title_case("jill is leaving A"));
//     println!("{}",change_case("heLLo THere"));
// }

// Joe is missing
// Jill Is Leaving A
// HEllO thERE

