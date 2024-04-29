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

pub struct Flag(){
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(s1: &String, s2: &String) -> Flag {
        Flag{
            short_hand: format!("-{}", s1.chars().next().unwrap()),
            long_hand: format!("--{}", s1),
            desc: format!("{}", s2)
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.insert.flags(flag, Callback)
    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        let func_cb = self.flags.get(&flag).unwrap();
        func_cb(arg[0], arg[1]).unwrap_or_else(|err| err.to_string())
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    a.parse::<f32>()? /  b.parse::<f32>()?

}
pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    a.parse::<f32>()? %  b.parse::<f32>()?
}
