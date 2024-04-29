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
use std::num::ParseFloatError;

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(s1: &str, s2: &str) -> Flag {
        Flag {
            short_hand: format!("-{}", s1.chars().next().unwrap()),
            long_hand: format!("--{}", s1),
            desc: format!("{}", s2),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }

    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        match self.flags.get(&flag) {
            Some(func_cb) => match func_cb(argv[0], argv[1]) {
                Ok(result) => result,
                Err(err) => err.to_string(),
            },
            None => String::from("Function not found"),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a = a.parse::<f32>()?;
    let b = b.parse::<f32>()?;
    Ok((a / b).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a = a.parse::<f32>()?;
    let b = b.parse::<f32>()?;
    Ok((a % b).to_string())
}