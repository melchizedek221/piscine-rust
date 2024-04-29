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

pub use chrono::{Utc, NaiveDate};

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    form_values: (String, String),
    date: String,
    err: String,
}

impl FormError {
    pub fn new(field_name: String, field_value: String, err: String, date: String) -> FormError {
        Self {
            form_values: (field_name, field_value),
            date,
            err,
        }
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    first_name: String,
    last_name: String,
    birth: NaiveDate,
    birth_location: String,
    password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Self {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }
    
    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        let mut errors: Vec<&str> = Vec::new();

        if self.first_name.is_empty() {
            errors.push("No user name");
        }
        if self.password.len() < 8 {
            errors.push("At least 8 characters");
        }
        
        if !self.password.chars().any(|c| c.is_digit(10))
            || !self.password.chars().any(char::is_alphabetic)
            || !self.password.chars().any(|c| !c.is_ascii_alphanumeric())
        {
            errors.push("Combination of different ASCII character types (numbers, letters and non-alphanumeric characters)");
        }
        
        if errors.is_empty() {
            Ok(vec!["Valid first name", "Valid password"])
        } else {
            let now = Utc::now();
            let date = now.format("%Y-%m-%d %H:%M:%S").to_string();
            Err(FormError::new("first_name".to_string(), &self.first_name, errors.join(", "), date))
        }
        
    }
}
// pub fn create_date(date: &str) -> NaiveDate {
//     NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
// }

// fn main() {
//     let mut form_output = Form::new(
//         String::from("Lee"),
//         String::from("Silva"),
//         create_date("2015-09-05"),
//         String::from("Africa"),
//         String::from("qwqwsa1dty_"),
//     );

//     println!("{:?}", form_output);
//     println!("{:?}", form_output.validate().unwrap());

//     form_output.first_name = String::from("");
//     println!("{:?}", form_output.validate().unwrap_err());

//     form_output.first_name = String::from("as");
//     form_output.password = String::from("dty_1");
//     println!("{:?}", form_output.validate().unwrap_err());

//     form_output.password = String::from("asdasASd(_");
//     println!("{:?}", form_output.validate().unwrap_err());

//     form_output.password = String::from("asdasASd123SA");
//     println!("{:?}", form_output.validate().unwrap_err());
// }