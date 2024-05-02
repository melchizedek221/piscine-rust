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

pub fn pig_latin(text: &str) -> String {
    fn is_vowel(c: char) -> bool {
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
    }
    
    let mut new_string = text.to_string();
    let mut next_char = false;

    for (i, c) in text.chars().enumerate() {
        if next_char && (c == 'q' || c == 'u') {
            new_string.remove(0);
            println!("{}", new_string);
            new_string.push(c);
            if c == 'u' {
                next_char = false;
            }
        } else if !is_vowel(c) && text.len() > i + 2 && &text[i + 1..i + 3] == "qu" {
            new_string.remove(0);
            new_string.push(c);
            next_char = true;
        } else if i == 0 && is_vowel(c) {
            break;
        } else if !is_vowel(c) {
            new_string.remove(0);
            new_string.push(c);
        } else {
            break;
        }
    }
    new_string + "ay"
}



// fn main() {
    // println!("{}", pig_latin(&String::from("igloo")));
    // println!("{}", pig_latin(&String::from("apple")));
    // println!("{}", pig_latin(&String::from("hello")));
    // println!("{}", pig_latin(&String::from("square")));
    // println!("{}", pig_latin(&String::from("xenon")));
    // println!("{}", pig_latin(&String::from("chair")));
	// println!("{}", pig_latin(&String::from("queen")));
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