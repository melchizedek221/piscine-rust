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


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError { validation, expected }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original.is_empty() || ciphered.is_empty() {
        return None;
    }

    let atbash_cipher = original
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'c' } else { b'A' };
                let offset = c as u8 - base;
                let cipher: char = (b'z' - offset) as char;
                if c.is_ascii_uppercase() {
                    cipher.to_ascii_uppercase()
                } else {
                    cipher
                }
            } else {
                c
            }
        })
        .collect::<String>();

    if atbash_cipher == ciphered {
        Some(Ok(true))
    } else {
        let error = CipherError::new(false, atbash_cipher);
        Some(Err(error))
    }
}


fn main() {
    // println!("{:?}", cipher("1Hello 2world!", "1Svool 2dliow!"));
    // println!("{:?}", cipher("1Hello 2world!", "svool"));
    // println!("{:?}", cipher("", "svool"));
}