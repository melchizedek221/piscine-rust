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

pub fn arrange_phrase(phrase: &str) -> String {
    let mut words_with_positions: Vec<(&str, usize)> = phrase
        .split_whitespace()
        .map(|word| {
            let position: usize = word
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            (word, position)
        })
        .collect();

    words_with_positions.sort_by_key(|&(_, position)| position);
    // println!("{:?}", words_with_positions);
    words_with_positions
        .iter()
        .map(|&(word, _)| word.chars().filter(|c| !c.is_numeric()).collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}
