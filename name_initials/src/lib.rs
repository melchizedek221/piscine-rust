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

pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result = Vec::new();

    for name in names {
        let mut initials = String::new();

        let parts = name.split_whitespace().collect::<Vec<_>>();
        for (i, part) in parts.iter().enumerate() { // enumerate pour am indexou part
            let mut part_initials = part.chars();
            if let Some(first_char) = part_initials.next() {
                initials.push_str(&format!("{}{}", first_char, "."));
                if i != parts.len() - 1 {
                    initials.push(' ');
                }
            }
        }
        result.push(initials);
    }
    result
}

// fn main() {
//     let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
//     println!("{:?}", initials(names));
// }