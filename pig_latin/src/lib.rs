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

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn pig_latin(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut new = word.trim_start_matches(|ch| !VOWELS.contains(&ch));
            let mut removed = word.len() - new.len();
            if &word[removed.saturating_sub(1)..=removed] == "qu" {
                removed += 1;
                new = &word[removed..];
            }
            let consonants = &word[0..removed];
            String::with_capacity(word.len() + 2) + new + consonants + "ay"
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// fn main() {
//     println!("{}", pig_latin(&String::from("igloo")));
//     println!("{}", pig_latin(&String::from("apple")));
//     println!("{}", pig_latin(&String::from("hello")));
//     println!("{}", pig_latin(&String::from("square")));
//     println!("{}", pig_latin(&String::from("xenon")));
//     println!("{}", pig_latin(&String::from("chair")));
// 	println!("{}", pig_latin(&String::from("queen")));
// }
// And its output:

// $ cargo run
// iglooay
// appleay
// ellohay
// aresquay
// enonxay
// airchay
// eenquay
// $