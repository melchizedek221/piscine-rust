pub fn rotate(input: &str, key: i8) -> String {
    let key = ((key % 26) + 26) % 26;
    let mut result = String::new();

    for ch in input.chars() {
        let rotated_char = match ch {
            'a'..='z' => {
                let new_char = ch as u8 + key as u8;
                if new_char > b'z' {
                    (new_char - 26) as char
                } else {
                    new_char as char
                }
            }
            'A'..='Z' => {
                let new_char = ch as u8 + key as u8;
                if new_char > b'Z' {
                    (new_char - 26) as char
                } else {
                    new_char as char
                }
            }
            _ => ch,
        };
        result.push(rotated_char);
    }

    result
}